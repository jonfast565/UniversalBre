use code_generation::mir::{MirInstruction, MirInstructionBlock, MirInstructions, MirOperator};
use semantic_analysis::data_types::DataType;
use semantic_analysis::operation_types::OperationType;

use std::collections::{HashMap, HashSet};

pub struct FasmGenerator {
    pub debug: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ValueKind {
    Integer,
    Float,
    Boolean,
    String,
    Unknown,
}

impl ValueKind {
    fn from_data_type(data_type: &DataType) -> ValueKind {
        match data_type {
            DataType::IntegerType => ValueKind::Integer,
            DataType::FloatType => ValueKind::Float,
            DataType::BooleanType => ValueKind::Boolean,
            DataType::StringType => ValueKind::String,
            DataType::AnyType => ValueKind::Unknown,
        }
    }
}

struct StackAllocator {
    slots: HashMap<String, i32>,
    next_offset: i32,
}

impl StackAllocator {
    fn new() -> StackAllocator {
        StackAllocator {
            slots: HashMap::new(),
            next_offset: 0,
        }
    }

    fn ensure_slot(&mut self, name: &str) -> i32 {
        if let Some(offset) = self.slots.get(name) {
            return *offset;
        }
        self.next_offset += 8;
        self.slots.insert(name.to_string(), self.next_offset);
        self.next_offset
    }

    fn get_slot(&self, name: &str) -> Option<i32> {
        self.slots.get(name).copied()
    }

    fn stack_size(&self) -> i32 {
        let raw = self.next_offset;
        if raw % 16 == 0 {
            raw
        } else {
            raw + (16 - (raw % 16))
        }
    }
}

struct FasmBuilder {
    data: Vec<String>,
    text: Vec<String>,
    float_literals: HashMap<String, String>,
    float_counter: usize,
    string_literals: HashMap<String, String>,
    string_counter: usize,
    externs: HashSet<String>,
}

impl FasmBuilder {
    fn new() -> FasmBuilder {
        FasmBuilder {
            data: Vec::new(),
            text: Vec::new(),
            float_literals: HashMap::new(),
            float_counter: 0,
            string_literals: HashMap::new(),
            string_counter: 0,
            externs: HashSet::new(),
        }
    }

    fn emit_text(&mut self, line: &str) {
        self.text.push(line.to_string());
    }

    fn emit_data(&mut self, line: &str) {
        self.data.push(line.to_string());
    }

    fn require_extern(&mut self, name: &str) {
        self.externs.insert(name.to_string());
    }

    fn get_or_insert_float_literal(&mut self, value: &str) -> String {
        if let Some(label) = self.float_literals.get(value) {
            return label.clone();
        }
        let label = format!("flt_{}", self.float_counter);
        self.float_counter += 1;
        self.emit_data(&format!("{} dq {}", label, value));
        self.float_literals.insert(value.to_string(), label.clone());
        label
    }

    fn get_or_insert_string_literal(&mut self, value: &str) -> String {
        if let Some(label) = self.string_literals.get(value) {
            return label.clone();
        }
        let label = format!("str_{}", self.string_counter);
        self.string_counter += 1;
        let bytes = value.as_bytes();
        let mut parts: Vec<String> = bytes.iter().map(|b| b.to_string()).collect();
        parts.push("0".to_string());
        let payload = parts.join(", ");
        self.emit_data(&format!("{} db {}", label, payload));
        self.string_literals.insert(value.to_string(), label.clone());
        label
    }

    fn finish(self) -> String {
        let mut output = String::new();
        output.push_str("format ELF64\n");
        output.push_str("entry _start\n");
        if !self.externs.is_empty() {
            let mut externs: Vec<String> = self.externs.into_iter().collect();
            externs.sort();
            for ext in externs {
                output.push_str(&format!("extrn {}\n", ext));
            }
        }
        output.push('\n');
        if !self.data.is_empty() {
            output.push_str("section '.data' writeable\n");
            for line in self.data {
                output.push_str(&format!("  {}\n", line));
            }
            output.push('\n');
        }
        output.push_str("section '.text' executable\n");
        output.push_str("_start:\n");
        for line in self.text {
            output.push_str(&format!("  {}\n", line));
        }
        output
    }
}

impl FasmGenerator {
    pub fn generate_asm(&self, p: &MirInstructions) -> String {
        let mut builder = FasmBuilder::new();
        let mut allocator = StackAllocator::new();
        let blocks = p.get_blocks();
        self.preallocate_slots(&mut allocator, &blocks);

        builder.emit_text("push rbp");
        builder.emit_text("mov rbp, rsp");
        if allocator.stack_size() > 0 {
            builder.emit_text(&format!("sub rsp, {}", allocator.stack_size()));
        }

        for block in blocks {
            self.codegen_block(&mut builder, &mut allocator, &block);
        }

        builder.emit_text("mov rax, 60");
        builder.emit_text("xor rdi, rdi");
        builder.emit_text("syscall");

        let program = builder.finish();
        if self.debug {
            dbg!(program.clone());
        }
        program
    }

    fn preallocate_slots(&self, allocator: &mut StackAllocator, blocks: &[MirInstructionBlock]) {
        for block in blocks {
            if let Some(target) = block.get_assignment_variable() {
                allocator.ensure_slot(&target.get_assignment_var_name());
            }
            for instruction in block.get_instructions() {
                self.reserve_instruction_slots(allocator, &instruction);
            }
        }
    }

    fn reserve_instruction_slots(&self, allocator: &mut StackAllocator, instruction: &MirInstruction) {
        if let Some(result) = &instruction.result_operator {
            if let Some(name) = &result.variable_identifier {
                allocator.ensure_slot(name);
            }
        }
        for operand in [&instruction.first_arg_operator, &instruction.second_arg_operator] {
            if let Some(op) = operand {
                if let Some(name) = &op.variable_identifier {
                    allocator.ensure_slot(name);
                }
            }
        }
    }

    fn codegen_block(
        &self,
        builder: &mut FasmBuilder,
        allocator: &mut StackAllocator,
        block: &MirInstructionBlock,
    ) {
        let instructions = block.get_instructions();
        let mut last_result: Option<MirOperator> = None;

        for instruction in instructions {
            self.codegen_instruction(builder, allocator, &instruction);
            last_result = instruction.result_operator.clone();
        }

        if let Some(target) = block.get_assignment_variable() {
            if let Some(result) = last_result {
                self.store_result_to_target(builder, allocator, &result, &target.get_assignment_var_name());
            }
        }
    }

    fn codegen_instruction(
        &self,
        builder: &mut FasmBuilder,
        allocator: &mut StackAllocator,
        instruction: &MirInstruction,
    ) {
        match instruction.operand {
            OperationType::NoOperation => {
                if let Some(result) = &instruction.result_operator {
                    if let Some(id) = &result.variable_identifier {
                        let offset = allocator.ensure_slot(id);
                        self.store_literal_or_variable(builder, allocator, result, offset);
                    }
                }
            }
            _ => {
                let result = instruction.result_operator.clone();
                let lhs = instruction.first_arg_operator.clone();
                let rhs = instruction.second_arg_operator.clone();
                if let (Some(result_op), Some(lhs_op), Some(rhs_op)) = (result, lhs, rhs) {
                    let result_slot = result_op
                        .variable_identifier
                        .clone()
                        .map(|name| allocator.ensure_slot(&name));
                    self.emit_binary(builder, allocator, instruction.operand, &lhs_op, &rhs_op, result_slot);
                }
            }
        }
    }

    fn store_result_to_target(
        &self,
        builder: &mut FasmBuilder,
        allocator: &mut StackAllocator,
        result: &MirOperator,
        target_name: &str,
    ) {
        let target_offset = allocator.ensure_slot(target_name);
        if let Some(literal) = &result.literal_value {
            let kind = ValueKind::from_data_type(&result.operator_type);
            match kind {
                ValueKind::Float => {
                    let label = builder.get_or_insert_float_literal(literal);
                    builder.emit_text(&format!("movsd xmm0, [rel {}]", label));
                    builder.emit_text(&format!("movsd qword [rbp-{}], xmm0", target_offset));
                }
                ValueKind::String => {
                    let label = builder.get_or_insert_string_literal(&unquote_string(literal));
                    builder.emit_text(&format!("lea rax, [rel {}]", label));
                    builder.emit_text(&format!("mov qword [rbp-{}], rax", target_offset));
                }
                ValueKind::Boolean => {
                    let value = if literal == "true" { "1" } else { "0" };
                    builder.emit_text(&format!("mov qword [rbp-{}], {}", target_offset, value));
                }
                _ => {
                    builder.emit_text(&format!("mov qword [rbp-{}], {}", target_offset, literal));
                }
            }
        } else if let Some(source_name) = &result.variable_identifier {
            if let Some(source_offset) = allocator.get_slot(source_name) {
                let kind = ValueKind::from_data_type(&result.operator_type);
                match kind {
                    ValueKind::Float => {
                        builder.emit_text(&format!("movsd xmm0, qword [rbp-{}]", source_offset));
                        builder.emit_text(&format!("movsd qword [rbp-{}], xmm0", target_offset));
                    }
                    _ => {
                        builder.emit_text(&format!("mov rax, qword [rbp-{}]", source_offset));
                        builder.emit_text(&format!("mov qword [rbp-{}], rax", target_offset));
                    }
                }
            }
        }
    }

    fn store_literal_or_variable(
        &self,
        builder: &mut FasmBuilder,
        allocator: &mut StackAllocator,
        result: &MirOperator,
        offset: i32,
    ) {
        if let Some(literal) = &result.literal_value {
            let kind = ValueKind::from_data_type(&result.operator_type);
            match kind {
                ValueKind::Float => {
                    let label = builder.get_or_insert_float_literal(literal);
                    builder.emit_text(&format!("movsd xmm0, [rel {}]", label));
                    builder.emit_text(&format!("movsd qword [rbp-{}], xmm0", offset));
                }
                ValueKind::String => {
                    let label = builder.get_or_insert_string_literal(&unquote_string(literal));
                    builder.emit_text(&format!("lea rax, [rel {}]", label));
                    builder.emit_text(&format!("mov qword [rbp-{}], rax", offset));
                }
                ValueKind::Boolean => {
                    let value = if literal == "true" { "1" } else { "0" };
                    builder.emit_text(&format!("mov qword [rbp-{}], {}", offset, value));
                }
                _ => {
                    builder.emit_text(&format!("mov qword [rbp-{}], {}", offset, literal));
                }
            }
        } else if let Some(source_name) = &result.variable_identifier {
            if let Some(source_offset) = allocator.get_slot(source_name) {
                builder.emit_text(&format!("mov rax, qword [rbp-{}]", source_offset));
                builder.emit_text(&format!("mov qword [rbp-{}], rax", offset));
            }
        }
    }

    fn emit_binary(
        &self,
        builder: &mut FasmBuilder,
        allocator: &mut StackAllocator,
        op: OperationType,
        lhs: &MirOperator,
        rhs: &MirOperator,
        result_slot: Option<i32>,
    ) {
        let lhs_kind = ValueKind::from_data_type(&lhs.operator_type);
        let rhs_kind = ValueKind::from_data_type(&rhs.operator_type);
        let float_op = lhs_kind == ValueKind::Float || rhs_kind == ValueKind::Float;

        match op {
            OperationType::AdditionOperation
            | OperationType::SubtractionOperation
            | OperationType::MultiplicationOperation
            | OperationType::DivisionOperation => {
                if float_op {
                    self.emit_float_binary(builder, allocator, op, lhs, rhs, result_slot);
                } else {
                    self.emit_int_binary(builder, allocator, op, lhs, rhs, result_slot);
                }
            }
            OperationType::BooleanAndOperation | OperationType::BooleanOrOperation => {
                self.emit_bool_binary(builder, allocator, op, lhs, rhs, result_slot);
            }
            OperationType::BooleanEqOperation
            | OperationType::BooleanNeOperation
            | OperationType::BooleanGtOperation
            | OperationType::BooleanGteOperation
            | OperationType::BooleanLtOperation
            | OperationType::BooleanLteOperation => {
                if lhs_kind == ValueKind::String || rhs_kind == ValueKind::String {
                    self.emit_string_compare(builder, allocator, op, lhs, rhs, result_slot);
                } else if float_op {
                    self.emit_float_compare(builder, allocator, op, lhs, rhs, result_slot);
                } else {
                    self.emit_int_compare(builder, allocator, op, lhs, rhs, result_slot);
                }
            }
            OperationType::ConcatOperation => {
                self.emit_string_concat(builder, allocator, lhs, rhs, result_slot);
            }
            OperationType::NoOperation => {}
        }
    }

    fn emit_int_binary(
        &self,
        builder: &mut FasmBuilder,
        allocator: &mut StackAllocator,
        op: OperationType,
        lhs: &MirOperator,
        rhs: &MirOperator,
        result_slot: Option<i32>,
    ) {
        self.load_int_operand(builder, allocator, lhs, "rax");
        self.load_int_operand(builder, allocator, rhs, "rcx");
        match op {
            OperationType::AdditionOperation => builder.emit_text("add rax, rcx"),
            OperationType::SubtractionOperation => builder.emit_text("sub rax, rcx"),
            OperationType::MultiplicationOperation => builder.emit_text("imul rax, rcx"),
            OperationType::DivisionOperation => {
                builder.emit_text("cqo");
                builder.emit_text("idiv rcx");
            }
            _ => {}
        }
        if let Some(slot) = result_slot {
            builder.emit_text(&format!("mov qword [rbp-{}], rax", slot));
        }
    }

    fn emit_float_binary(
        &self,
        builder: &mut FasmBuilder,
        allocator: &mut StackAllocator,
        op: OperationType,
        lhs: &MirOperator,
        rhs: &MirOperator,
        result_slot: Option<i32>,
    ) {
        self.load_float_operand(builder, allocator, lhs, "xmm0");
        self.load_float_operand(builder, allocator, rhs, "xmm1");
        match op {
            OperationType::AdditionOperation => builder.emit_text("addsd xmm0, xmm1"),
            OperationType::SubtractionOperation => builder.emit_text("subsd xmm0, xmm1"),
            OperationType::MultiplicationOperation => builder.emit_text("mulsd xmm0, xmm1"),
            OperationType::DivisionOperation => builder.emit_text("divsd xmm0, xmm1"),
            _ => {}
        }
        if let Some(slot) = result_slot {
            builder.emit_text(&format!("movsd qword [rbp-{}], xmm0", slot));
        }
    }

    fn emit_bool_binary(
        &self,
        builder: &mut FasmBuilder,
        allocator: &mut StackAllocator,
        op: OperationType,
        lhs: &MirOperator,
        rhs: &MirOperator,
        result_slot: Option<i32>,
    ) {
        self.load_int_operand(builder, allocator, lhs, "rax");
        self.load_int_operand(builder, allocator, rhs, "rcx");
        match op {
            OperationType::BooleanAndOperation => builder.emit_text("and rax, rcx"),
            OperationType::BooleanOrOperation => builder.emit_text("or rax, rcx"),
            _ => {}
        }
        if let Some(slot) = result_slot {
            builder.emit_text(&format!("mov qword [rbp-{}], rax", slot));
        }
    }

    fn emit_int_compare(
        &self,
        builder: &mut FasmBuilder,
        allocator: &mut StackAllocator,
        op: OperationType,
        lhs: &MirOperator,
        rhs: &MirOperator,
        result_slot: Option<i32>,
    ) {
        self.load_int_operand(builder, allocator, lhs, "rax");
        self.load_int_operand(builder, allocator, rhs, "rcx");
        builder.emit_text("cmp rax, rcx");
        let set_instr = match op {
            OperationType::BooleanEqOperation => "sete",
            OperationType::BooleanNeOperation => "setne",
            OperationType::BooleanGtOperation => "setg",
            OperationType::BooleanGteOperation => "setge",
            OperationType::BooleanLtOperation => "setl",
            OperationType::BooleanLteOperation => "setle",
            _ => "sete",
        };
        builder.emit_text(&format!("{} al", set_instr));
        builder.emit_text("movzx rax, al");
        if let Some(slot) = result_slot {
            builder.emit_text(&format!("mov qword [rbp-{}], rax", slot));
        }
    }

    fn emit_float_compare(
        &self,
        builder: &mut FasmBuilder,
        allocator: &mut StackAllocator,
        op: OperationType,
        lhs: &MirOperator,
        rhs: &MirOperator,
        result_slot: Option<i32>,
    ) {
        self.load_float_operand(builder, allocator, lhs, "xmm0");
        self.load_float_operand(builder, allocator, rhs, "xmm1");
        builder.emit_text("ucomisd xmm0, xmm1");
        let set_instr = match op {
            OperationType::BooleanEqOperation => "sete",
            OperationType::BooleanNeOperation => "setne",
            OperationType::BooleanGtOperation => "seta",
            OperationType::BooleanGteOperation => "setae",
            OperationType::BooleanLtOperation => "setb",
            OperationType::BooleanLteOperation => "setbe",
            _ => "sete",
        };
        builder.emit_text(&format!("{} al", set_instr));
        builder.emit_text("movzx rax, al");
        if let Some(slot) = result_slot {
            builder.emit_text(&format!("mov qword [rbp-{}], rax", slot));
        }
    }

    fn emit_string_compare(
        &self,
        builder: &mut FasmBuilder,
        allocator: &mut StackAllocator,
        op: OperationType,
        lhs: &MirOperator,
        rhs: &MirOperator,
        result_slot: Option<i32>,
    ) {
        if op != OperationType::BooleanEqOperation && op != OperationType::BooleanNeOperation {
            if let Some(slot) = result_slot {
                builder.emit_text(&format!("mov qword [rbp-{}], 0", slot));
            }
            return;
        }
        builder.require_extern("ub_str_eq");
        self.load_ptr_operand(builder, allocator, lhs, "rdi");
        self.load_ptr_operand(builder, allocator, rhs, "rsi");
        builder.emit_text("call ub_str_eq");
        if op == OperationType::BooleanNeOperation {
            builder.emit_text("xor rax, 1");
        }
        if let Some(slot) = result_slot {
            builder.emit_text(&format!("mov qword [rbp-{}], rax", slot));
        }
    }

    fn emit_string_concat(
        &self,
        builder: &mut FasmBuilder,
        allocator: &mut StackAllocator,
        lhs: &MirOperator,
        rhs: &MirOperator,
        result_slot: Option<i32>,
    ) {
        if let Some(slot) = result_slot {
            builder.require_extern("ub_str_concat");
            self.load_ptr_operand(builder, allocator, lhs, "rdi");
            self.load_ptr_operand(builder, allocator, rhs, "rsi");
            builder.emit_text("call ub_str_concat");
            builder.emit_text(&format!("mov qword [rbp-{}], rax", slot));
        }
    }

    fn load_int_operand(
        &self,
        builder: &mut FasmBuilder,
        allocator: &mut StackAllocator,
        operand: &MirOperator,
        reg: &str,
    ) {
        if let Some(literal) = &operand.literal_value {
            let kind = ValueKind::from_data_type(&operand.operator_type);
            let literal = if kind == ValueKind::Boolean {
                if literal == "true" { "1" } else { "0" }
            } else {
                literal
            };
            builder.emit_text(&format!("mov {}, {}", reg, literal));
        } else if let Some(name) = &operand.variable_identifier {
            let offset = allocator.ensure_slot(name);
            builder.emit_text(&format!("mov {}, qword [rbp-{}]", reg, offset));
        }
    }

    fn load_float_operand(
        &self,
        builder: &mut FasmBuilder,
        allocator: &mut StackAllocator,
        operand: &MirOperator,
        reg: &str,
    ) {
        if let Some(literal) = &operand.literal_value {
            let label = builder.get_or_insert_float_literal(literal);
            builder.emit_text(&format!("movsd {}, [rel {}]", reg, label));
        } else if let Some(name) = &operand.variable_identifier {
            let offset = allocator.ensure_slot(name);
            builder.emit_text(&format!("movsd {}, qword [rbp-{}]", reg, offset));
        }
    }

    fn load_ptr_operand(
        &self,
        builder: &mut FasmBuilder,
        allocator: &mut StackAllocator,
        operand: &MirOperator,
        reg: &str,
    ) {
        if let Some(literal) = &operand.literal_value {
            let label = builder.get_or_insert_string_literal(&unquote_string(literal));
            builder.emit_text(&format!("lea {}, [rel {}]", reg, label));
        } else if let Some(name) = &operand.variable_identifier {
            let offset = allocator.ensure_slot(name);
            builder.emit_text(&format!("mov {}, qword [rbp-{}]", reg, offset));
        }
    }
}

fn unquote_string(value: &str) -> String {
    if value.len() >= 2 && value.starts_with('\"') && value.ends_with('\"') {
        value[1..value.len() - 1].to_string()
    } else {
        value.to_string()
    }
}
