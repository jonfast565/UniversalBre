use semantic_analysis::data_types::DataType;
use semantic_analysis::expressions::{ExprNode, ExprType};
use semantic_analysis::functions::FunctionBlock;
use semantic_analysis::loops::{LoopBlock, LoopType};
use semantic_analysis::operation_types::OperationType;
use semantic_analysis::program::Program;
use semantic_analysis::semantic_blocks::{BlockType, SemanticBlock};
use semantic_analysis::statements::{AssignmentBlock, BreakBlock, ReturnBlock};

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LlvmType {
    Void,
    I1,
    I64,
    Double,
    I8Ptr,
}

impl LlvmType {
    fn as_ir(&self) -> &'static str {
        match self {
            LlvmType::Void => "void",
            LlvmType::I1 => "i1",
            LlvmType::I64 => "i64",
            LlvmType::Double => "double",
            LlvmType::I8Ptr => "i8*",
        }
    }

    fn zero_literal(&self) -> &'static str {
        match self {
            LlvmType::Void => "",
            LlvmType::I1 => "0",
            LlvmType::I64 => "0",
            LlvmType::Double => "0.0",
            LlvmType::I8Ptr => "null",
        }
    }
}

#[derive(Clone, Debug)]
struct LlvmValue {
    ty: LlvmType,
    name: String,
}

impl LlvmValue {
    fn new(ty: LlvmType, name: String) -> LlvmValue {
        LlvmValue { ty, name }
    }
}

#[derive(Clone, Debug)]
struct LlvmLocal {
    ptr: String,
    ty: LlvmType,
}

#[derive(Clone, Debug)]
struct LoopLabels {
    start_label: String,
    end_label: String,
}

pub struct LlvmGenerator {
    pub debug: bool,
}

impl LlvmGenerator {
    pub fn generate_ir(&self, program: &Program) -> String {
        let mut module = LlvmModule::new();
        let mut context = LlvmContext::new();

        for block in program.get_blocks() {
            match block.get_block_type() {
                BlockType::FunctionBlock => {
                    let function = block.get_function_block();
                    module.add_function(self.codegen_function(&mut context, &function));
                }
                _ => {}
            }
        }

        // Top-level blocks become main.
        let main_block = program.get_blocks();
        if !main_block.is_empty() {
            module.add_function(self.codegen_main(&mut context, &main_block));
        }

        module.add_globals(context.globals);
        module.add_externs(context.externs);
        module.render()
    }

    fn codegen_main(&self, context: &mut LlvmContext, blocks: &[SemanticBlock]) -> LlvmFunction {
        let mut builder = LlvmFunctionBuilder::new("main".to_string(), LlvmType::I64);
        builder.start_entry();
        self.codegen_blocks(context, &mut builder, blocks);

        if !builder.is_terminated() {
            builder.emit("ret i64 0");
        }

        builder.finish()
    }

    fn codegen_function(&self, context: &mut LlvmContext, function: &FunctionBlock) -> LlvmFunction {
        let return_ty = infer_return_type(function).unwrap_or(LlvmType::I64);
        let mut builder = LlvmFunctionBuilder::new(function.get_name(), return_ty);

        let args = function.get_arguments();
        for (index, arg) in args.iter().enumerate() {
            let arg_name = format!("%arg{}", index);
            builder.add_param(arg_name.clone(), LlvmType::I64);
            let alloca = builder.alloca(&arg.get_name(), LlvmType::I64);
            builder.emit_entry(&format!("store i64 {}, i64* {}", arg_name, alloca));
        }

        builder.start_entry();
        self.codegen_blocks(context, &mut builder, &function.get_body());
        if !builder.is_terminated() {
            match return_ty {
                LlvmType::Void => builder.emit("ret void"),
                _ => builder.emit(&format!("ret {} {}", return_ty.as_ir(), return_ty.zero_literal())),
            }
        }

        builder.finish()
    }

    fn codegen_blocks(
        &self,
        context: &mut LlvmContext,
        builder: &mut LlvmFunctionBuilder,
        blocks: &[SemanticBlock],
    ) {
        for block in blocks {
            match block.get_block_type() {
                BlockType::AssignmentBlock => {
                    self.codegen_assignment(context, builder, block.get_assignment_block());
                }
                BlockType::ReturnBlock => {
                    self.codegen_return(context, builder, block.get_return_block());
                }
                BlockType::BreakBlock => {
                    self.codegen_break(builder, block.get_break_block());
                }
                BlockType::LoopBlock => {
                    self.codegen_loop(context, builder, block.get_loop_block());
                }
                BlockType::FunctionBlock => {}
            }
            if builder.is_terminated() {
                break;
            }
        }
    }

    fn codegen_assignment(
        &self,
        context: &mut LlvmContext,
        builder: &mut LlvmFunctionBuilder,
        assignment: AssignmentBlock,
    ) {
        let expr = assignment.get_expression().unwrap();
        let value = self.codegen_expr(context, builder, &expr);
        let target_name = assignment.get_assignment_id();

        let local = if let Some(existing) = builder.locals.get(&target_name) {
            existing.clone()
        } else {
            let alloca = builder.alloca(&target_name, value.ty);
            let local = LlvmLocal { ptr: alloca, ty: value.ty };
            builder.locals.insert(target_name.clone(), local.clone());
            local
        };

        let coerced = coerce_value(builder, value, local.ty);
        builder.emit(&format!(
            "store {} {}, {}* {}",
            local.ty.as_ir(),
            coerced.name,
            local.ty.as_ir(),
            local.ptr
        ));
    }

    fn codegen_return(
        &self,
        context: &mut LlvmContext,
        builder: &mut LlvmFunctionBuilder,
        ret: ReturnBlock,
    ) {
        let value = match ret.return_expression {
            Some(expr) => self.codegen_expr(context, builder, &expr),
            None => {
                builder.emit("ret void");
                builder.terminate();
                return;
            }
        };

        let coerced = coerce_value(builder, value, builder.return_type);
        builder.emit(&format!("ret {} {}", builder.return_type.as_ir(), coerced.name));
        builder.terminate();
    }

    fn codegen_break(&self, builder: &mut LlvmFunctionBuilder, _breaker: BreakBlock) {
        if let Some(loop_label) = builder.loop_stack.last() {
            builder.emit(&format!("br label %{}", loop_label.end_label));
            builder.terminate();
        }
    }

    fn codegen_loop(
        &self,
        context: &mut LlvmContext,
        builder: &mut LlvmFunctionBuilder,
        loop_block: LoopBlock,
    ) {
        match loop_block.get_loop_type() {
            LoopType::InfiniteLoop => {
                let start_label = builder.new_label("loop_start");
                let end_label = builder.new_label("loop_end");

                builder.emit(&format!("br label %{}", start_label));
                builder.emit_label(&start_label);

                builder.loop_stack.push(LoopLabels { start_label: start_label.clone(), end_label: end_label.clone() });
                self.codegen_blocks(context, builder, &loop_block.get_statement_list());
                builder.loop_stack.pop();

                if !builder.is_terminated() {
                    builder.emit(&format!("br label %{}", start_label));
                }

                builder.emit_label(&end_label);
                builder.clear_termination();
            }
        }
    }

    fn codegen_expr(
        &self,
        context: &mut LlvmContext,
        builder: &mut LlvmFunctionBuilder,
        expr: &Arc<ExprNode>,
    ) -> LlvmValue {
        match expr.get_expression_type() {
            ExprType::Literal => self.codegen_literal(context, builder, expr),
            ExprType::Variable => {
                if *expr.get_type() == DataType::StringType {
                    let value = expr.get_value();
                if value.starts_with('"') && value.ends_with('"') {
                    return self.codegen_string_literal(context, builder, &value);
                }
            }
            self.codegen_variable(builder, expr)
        }
            ExprType::Binary => {
                let left = expr.get_left_node().unwrap();
                let right = expr.get_right_node().unwrap();
                let op = *expr.get_operation_type();
                let left_value = self.codegen_expr(context, builder, &left);
                let right_value = self.codegen_expr(context, builder, &right);
                self.codegen_binary(context, builder, op, left_value, right_value)
            }
        }
    }

    fn codegen_literal(
        &self,
        context: &mut LlvmContext,
        builder: &mut LlvmFunctionBuilder,
        expr: &Arc<ExprNode>,
    ) -> LlvmValue {
        match *expr.get_type() {
            DataType::IntegerType => LlvmValue::new(LlvmType::I64, expr.get_value()),
            DataType::FloatType => LlvmValue::new(LlvmType::Double, expr.get_value()),
            DataType::BooleanType => {
                let value = expr.get_value();
                let literal = if value == "true" { "1" } else { "0" };
                LlvmValue::new(LlvmType::I1, literal.to_string())
            }
            DataType::StringType => {
                let value = expr.get_value();
                self.codegen_string_literal(context, builder, &value)
            }
            DataType::AnyType => LlvmValue::new(LlvmType::I64, expr.get_value()),
        }
    }

    fn codegen_variable(&self, builder: &mut LlvmFunctionBuilder, expr: &Arc<ExprNode>) -> LlvmValue {
        let name = expr.get_value();
        if let Some(local) = builder.locals.get(&name) {
            let tmp = builder.new_temp();
            builder.emit(&format!(
                "{} = load {}, {}* {}",
                tmp,
                local.ty.as_ir(),
                local.ty.as_ir(),
                local.ptr
            ));
            LlvmValue::new(local.ty, tmp)
        } else {
            let alloca = builder.alloca(&name, LlvmType::I64);
            builder.emit(&format!("store i64 0, i64* {}", alloca));
            builder.locals.insert(name.clone(), LlvmLocal { ptr: alloca.clone(), ty: LlvmType::I64 });
            let tmp = builder.new_temp();
            builder.emit(&format!("{} = load i64, i64* {}", tmp, alloca));
            LlvmValue::new(LlvmType::I64, tmp)
        }
    }

    fn codegen_string_literal(
        &self,
        context: &mut LlvmContext,
        builder: &mut LlvmFunctionBuilder,
        raw_value: &str,
    ) -> LlvmValue {
        let unquoted = unquote(raw_value);
        let (label, length) = context.intern_string(&unquoted);
        let tmp = builder.new_temp();
        builder.emit(&format!(
            "{} = getelementptr inbounds ([{} x i8], [{} x i8]* {}, i64 0, i64 0)",
            tmp, length, length, label
        ));
        LlvmValue::new(LlvmType::I8Ptr, tmp)
    }

    fn codegen_binary(
        &self,
        context: &mut LlvmContext,
        builder: &mut LlvmFunctionBuilder,
        op: OperationType,
        left: LlvmValue,
        right: LlvmValue,
    ) -> LlvmValue {
        match op {
            OperationType::AdditionOperation
            | OperationType::SubtractionOperation
            | OperationType::MultiplicationOperation
            | OperationType::DivisionOperation => {
                let (lhs, rhs, ty) = coerce_numeric_pair(builder, left, right);
                let tmp = builder.new_temp();
                let instr = match (op, ty) {
                    (OperationType::AdditionOperation, LlvmType::Double) => "fadd",
                    (OperationType::SubtractionOperation, LlvmType::Double) => "fsub",
                    (OperationType::MultiplicationOperation, LlvmType::Double) => "fmul",
                    (OperationType::DivisionOperation, LlvmType::Double) => "fdiv",
                    (OperationType::AdditionOperation, _) => "add",
                    (OperationType::SubtractionOperation, _) => "sub",
                    (OperationType::MultiplicationOperation, _) => "mul",
                    (OperationType::DivisionOperation, _) => "sdiv",
                    _ => "add",
                };
                builder.emit(&format!(
                    "{} = {} {} {}, {}",
                    tmp,
                    instr,
                    ty.as_ir(),
                    lhs.name,
                    rhs.name
                ));
                LlvmValue::new(ty, tmp)
            }
            OperationType::BooleanAndOperation | OperationType::BooleanOrOperation => {
                let lhs = coerce_to_bool(builder, left);
                let rhs = coerce_to_bool(builder, right);
                let tmp = builder.new_temp();
                let instr = match op {
                    OperationType::BooleanAndOperation => "and",
                    OperationType::BooleanOrOperation => "or",
                    _ => "and",
                };
                builder.emit(&format!("{} = {} i1 {}, {}", tmp, instr, lhs.name, rhs.name));
                LlvmValue::new(LlvmType::I1, tmp)
            }
            OperationType::BooleanEqOperation | OperationType::BooleanNeOperation => {
                if left.ty == LlvmType::I8Ptr && right.ty == LlvmType::I8Ptr {
                context.require_extern(ExternFn::StrEq);
                let tmp = builder.new_temp();
                builder.emit(&format!(
                    "{} = call i1 @ub_str_eq(i8* {}, i8* {})",
                    tmp, left.name, right.name
                ));
                    if op == OperationType::BooleanNeOperation {
                        let not_tmp = builder.new_temp();
                        builder.emit(&format!("{} = xor i1 {}, true", not_tmp, tmp));
                        return LlvmValue::new(LlvmType::I1, not_tmp);
                    }
                    LlvmValue::new(LlvmType::I1, tmp)
                } else {
                    let (lhs, rhs, cmp_ty, is_float) = coerce_compare_pair(builder, left, right);
                    let tmp = builder.new_temp();
                    let pred = if op == OperationType::BooleanEqOperation { "eq" } else { "ne" };
                    if is_float {
                        builder.emit(&format!(
                            "{} = fcmp o{} {} {}, {}",
                            tmp,
                            pred,
                            cmp_ty.as_ir(),
                            lhs.name,
                            rhs.name
                        ));
                    } else {
                        builder.emit(&format!(
                            "{} = icmp {} {} {}, {}",
                            tmp,
                            pred,
                            cmp_ty.as_ir(),
                            lhs.name,
                            rhs.name
                        ));
                    }
                    LlvmValue::new(LlvmType::I1, tmp)
                }
            }
            OperationType::BooleanGtOperation
            | OperationType::BooleanGteOperation
            | OperationType::BooleanLtOperation
            | OperationType::BooleanLteOperation => {
                let (lhs, rhs, cmp_ty, is_float) = coerce_compare_pair(builder, left, right);
                let tmp = builder.new_temp();
                let pred = match op {
                    OperationType::BooleanGtOperation => "gt",
                    OperationType::BooleanGteOperation => "ge",
                    OperationType::BooleanLtOperation => "lt",
                    OperationType::BooleanLteOperation => "le",
                    _ => "gt",
                };
                if is_float {
                    builder.emit(&format!(
                        "{} = fcmp o{} {} {}, {}",
                        tmp,
                        pred,
                        cmp_ty.as_ir(),
                        lhs.name,
                        rhs.name
                    ));
                } else {
                    builder.emit(&format!(
                        "{} = icmp s{} {} {}, {}",
                        tmp,
                        pred,
                        cmp_ty.as_ir(),
                        lhs.name,
                        rhs.name
                    ));
                }
                LlvmValue::new(LlvmType::I1, tmp)
            }
            OperationType::ConcatOperation => {
                context.require_extern(ExternFn::StrConcat);
                let lhs = coerce_value(builder, left, LlvmType::I8Ptr);
                let rhs = coerce_value(builder, right, LlvmType::I8Ptr);
                let tmp = builder.new_temp();
                builder.emit(&format!(
                    "{} = call i8* @ub_str_concat(i8* {}, i8* {})",
                    tmp, lhs.name, rhs.name
                ));
                LlvmValue::new(LlvmType::I8Ptr, tmp)
            }
            OperationType::NoOperation => left,
        }
    }
}

struct LlvmModule {
    globals: Vec<String>,
    externs: HashSet<ExternFn>,
    functions: Vec<LlvmFunction>,
}

impl LlvmModule {
    fn new() -> LlvmModule {
        LlvmModule {
            globals: Vec::new(),
            externs: HashSet::new(),
            functions: Vec::new(),
        }
    }

    fn add_function(&mut self, function: LlvmFunction) {
        self.functions.push(function);
    }

    fn add_globals(&mut self, globals: Vec<String>) {
        for global in globals {
            self.globals.push(global);
        }
    }

    fn add_externs(&mut self, externs: HashSet<ExternFn>) {
        for ext in externs {
            self.externs.insert(ext);
        }
    }

    fn render(self) -> String {
        let mut result = String::new();
        for global in self.globals {
            result.push_str(&global);
            result.push('\n');
        }

        for extern_decl in self.externs {
            result.push_str(extern_decl.declaration());
            result.push('\n');
        }

        for function in self.functions {
            result.push_str(&function.render());
            result.push('\n');
        }

        result
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum ExternFn {
    StrConcat,
    StrEq,
}

impl ExternFn {
    fn declaration(&self) -> &'static str {
        match self {
            ExternFn::StrConcat => "declare i8* @ub_str_concat(i8*, i8*)",
            ExternFn::StrEq => "declare i1 @ub_str_eq(i8*, i8*)",
        }
    }
}

struct LlvmContext {
    string_map: HashMap<String, (String, usize)>,
    string_counter: usize,
    externs: HashSet<ExternFn>,
    globals: Vec<String>,
}

impl LlvmContext {
    fn new() -> LlvmContext {
        LlvmContext {
            string_map: HashMap::new(),
            string_counter: 0,
            externs: HashSet::new(),
            globals: Vec::new(),
        }
    }

    fn intern_string(&mut self, value: &str) -> (String, usize) {
        if let Some(existing) = self.string_map.get(value) {
            return existing.clone();
        }

        let label = format!("@.str.{}", self.string_counter);
        self.string_counter += 1;

        let escaped = escape_string(value);
        let length = escaped.len() + 1;
        let global = format!(
            "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"",
            label, length, escaped
        );
        self.string_map.insert(value.to_string(), (label.clone(), length));
        self.globals.push(global);
        (label, length)
    }

    fn require_extern(&mut self, ext: ExternFn) {
        self.externs.insert(ext);
    }
}

struct LlvmFunction {
    name: String,
    return_type: LlvmType,
    params: Vec<(String, LlvmType)>,
    entry_allocas: Vec<String>,
    body: Vec<String>,
}

impl LlvmFunction {
    fn render(&self) -> String {
        let params = self
            .params
            .iter()
            .map(|(name, ty)| format!("{} {}", ty.as_ir(), name))
            .collect::<Vec<String>>()
            .join(", ");
        let mut result = String::new();
        result.push_str(&format!(
            "define {} @{}({}) {{\n",
            self.return_type.as_ir(),
            self.name,
            params
        ));
        result.push_str("entry:\n");
        for line in &self.entry_allocas {
            result.push_str(&format!("  {}\n", line));
        }
        for line in &self.body {
            if line.ends_with(':') {
                result.push_str(&format!("{}\n", line));
            } else {
                result.push_str(&format!("  {}\n", line));
            }
        }
        result.push_str("}\n");
        result
    }
}

struct LlvmFunctionBuilder {
    name: String,
    return_type: LlvmType,
    params: Vec<(String, LlvmType)>,
    entry_allocas: Vec<String>,
    body: Vec<String>,
    locals: HashMap<String, LlvmLocal>,
    temp_counter: usize,
    label_counter: usize,
    terminated: bool,
    loop_stack: Vec<LoopLabels>,
}

impl LlvmFunctionBuilder {
    fn new(name: String, return_type: LlvmType) -> LlvmFunctionBuilder {
        LlvmFunctionBuilder {
            name,
            return_type,
            params: Vec::new(),
            entry_allocas: Vec::new(),
            body: Vec::new(),
            locals: HashMap::new(),
            temp_counter: 0,
            label_counter: 0,
            terminated: false,
            loop_stack: Vec::new(),
        }
    }

    fn start_entry(&mut self) {
        // Entry block is implicit, nothing to do here.
    }

    fn add_param(&mut self, name: String, ty: LlvmType) {
        self.params.push((name, ty));
    }

    fn alloca(&mut self, name: &str, ty: LlvmType) -> String {
        let ptr_name = format!("%{}", name);
        let line = format!("{} = alloca {}", ptr_name, ty.as_ir());
        self.entry_allocas.push(line);
        ptr_name
    }

    fn emit_entry(&mut self, line: &str) {
        self.entry_allocas.push(line.to_string());
    }

    fn new_temp(&mut self) -> String {
        let name = format!("%t{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }

    fn new_label(&mut self, prefix: &str) -> String {
        let label = format!("{}.{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    fn emit(&mut self, line: &str) {
        if !self.terminated {
            self.body.push(line.to_string());
        }
    }

    fn emit_label(&mut self, label: &str) {
        self.body.push(format!("{}:", label));
    }

    fn terminate(&mut self) {
        self.terminated = true;
    }

    fn clear_termination(&mut self) {
        self.terminated = false;
    }

    fn is_terminated(&self) -> bool {
        self.terminated
    }

    fn finish(self) -> LlvmFunction {
        LlvmFunction {
            name: self.name,
            return_type: self.return_type,
            params: self.params,
            entry_allocas: self.entry_allocas,
            body: self.body,
        }
    }
}

fn infer_return_type(function: &FunctionBlock) -> Option<LlvmType> {
    for block in function.get_body() {
        if block.get_block_type() == BlockType::ReturnBlock {
            let ret = block.get_return_block();
            if let Some(expr) = ret.return_expression {
                return Some(infer_expr_type(&expr));
            }
        }
    }
    None
}

fn infer_expr_type(expr: &Arc<ExprNode>) -> LlvmType {
    match expr.get_expression_type() {
        ExprType::Literal => map_type(expr.get_type()),
        ExprType::Variable => map_type(expr.get_type()),
        ExprType::Binary => {
            let left = expr.get_left_node().unwrap();
            let right = expr.get_right_node().unwrap();
            let op = *expr.get_operation_type();
            match op {
                OperationType::BooleanAndOperation
                | OperationType::BooleanOrOperation
                | OperationType::BooleanEqOperation
                | OperationType::BooleanNeOperation
                | OperationType::BooleanGtOperation
                | OperationType::BooleanGteOperation
                | OperationType::BooleanLtOperation
                | OperationType::BooleanLteOperation => LlvmType::I1,
                OperationType::ConcatOperation => LlvmType::I8Ptr,
                _ => {
                    let left_ty = infer_expr_type(&left);
                    let right_ty = infer_expr_type(&right);
                    if left_ty == LlvmType::Double || right_ty == LlvmType::Double {
                        LlvmType::Double
                    } else {
                        LlvmType::I64
                    }
                }
            }
        }
    }
}

fn map_type(data_type: Arc<DataType>) -> LlvmType {
    match *data_type {
        DataType::BooleanType => LlvmType::I1,
        DataType::FloatType => LlvmType::Double,
        DataType::IntegerType => LlvmType::I64,
        DataType::StringType => LlvmType::I8Ptr,
        DataType::AnyType => LlvmType::I64,
    }
}

fn coerce_value(builder: &mut LlvmFunctionBuilder, value: LlvmValue, target: LlvmType) -> LlvmValue {
    if value.ty == target {
        return value;
    }
    match (value.ty, target) {
        (LlvmType::I1, LlvmType::I64) => {
            let tmp = builder.new_temp();
            builder.emit(&format!("{} = zext i1 {} to i64", tmp, value.name));
            LlvmValue::new(LlvmType::I64, tmp)
        }
        (LlvmType::I64, LlvmType::I1) => {
            let tmp = builder.new_temp();
            builder.emit(&format!("{} = icmp ne i64 {}, 0", tmp, value.name));
            LlvmValue::new(LlvmType::I1, tmp)
        }
        (LlvmType::Double, LlvmType::I64) => {
            let tmp = builder.new_temp();
            builder.emit(&format!("{} = fptosi double {} to i64", tmp, value.name));
            LlvmValue::new(LlvmType::I64, tmp)
        }
        (LlvmType::I64, LlvmType::Double) => {
            let tmp = builder.new_temp();
            builder.emit(&format!("{} = sitofp i64 {} to double", tmp, value.name));
            LlvmValue::new(LlvmType::Double, tmp)
        }
        (LlvmType::I1, LlvmType::Double) => {
            let tmp = builder.new_temp();
            builder.emit(&format!("{} = uitofp i1 {} to double", tmp, value.name));
            LlvmValue::new(LlvmType::Double, tmp)
        }
        (LlvmType::Double, LlvmType::I1) => {
            let tmp = builder.new_temp();
            builder.emit(&format!("{} = fcmp one double {}, 0.0", tmp, value.name));
            LlvmValue::new(LlvmType::I1, tmp)
        }
        (LlvmType::I8Ptr, LlvmType::I1) => {
            let tmp = builder.new_temp();
            builder.emit(&format!("{} = icmp ne i8* {}, null", tmp, value.name));
            LlvmValue::new(LlvmType::I1, tmp)
        }
        (LlvmType::I1, LlvmType::I8Ptr) => value,
        (LlvmType::I64, LlvmType::I8Ptr) => value,
        (LlvmType::Double, LlvmType::I8Ptr) => value,
        _ => value,
    }
}

fn coerce_to_bool(builder: &mut LlvmFunctionBuilder, value: LlvmValue) -> LlvmValue {
    coerce_value(builder, value, LlvmType::I1)
}

fn coerce_numeric_pair(
    builder: &mut LlvmFunctionBuilder,
    left: LlvmValue,
    right: LlvmValue,
) -> (LlvmValue, LlvmValue, LlvmType) {
    if left.ty == LlvmType::Double || right.ty == LlvmType::Double {
        let lhs = coerce_value(builder, left, LlvmType::Double);
        let rhs = coerce_value(builder, right, LlvmType::Double);
        (lhs, rhs, LlvmType::Double)
    } else {
        let lhs = coerce_value(builder, left, LlvmType::I64);
        let rhs = coerce_value(builder, right, LlvmType::I64);
        (lhs, rhs, LlvmType::I64)
    }
}

fn coerce_compare_pair(
    builder: &mut LlvmFunctionBuilder,
    left: LlvmValue,
    right: LlvmValue,
) -> (LlvmValue, LlvmValue, LlvmType, bool) {
    if left.ty == LlvmType::Double || right.ty == LlvmType::Double {
        let lhs = coerce_value(builder, left, LlvmType::Double);
        let rhs = coerce_value(builder, right, LlvmType::Double);
        (lhs, rhs, LlvmType::Double, true)
    } else {
        let lhs = coerce_value(builder, left, LlvmType::I64);
        let rhs = coerce_value(builder, right, LlvmType::I64);
        (lhs, rhs, LlvmType::I64, false)
    }
}

fn unquote(value: &str) -> String {
    if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
        value[1..value.len() - 1].to_string()
    } else {
        value.to_string()
    }
}

fn escape_string(value: &str) -> String {
    let mut escaped = String::new();
    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\5C"),
            '"' => escaped.push_str("\\22"),
            '\n' => escaped.push_str("\\0A"),
            '\r' => escaped.push_str("\\0D"),
            '\t' => escaped.push_str("\\09"),
            _ => escaped.push(ch),
        }
    }
    escaped
}
