use semantic_analysis::expressions::ExprNode;
use semantic_analysis::functions::FunctionBlock;
use semantic_analysis::loops::LoopBlock;
use semantic_analysis::program::Program;
use semantic_analysis::semantic_blocks::{BlockType, SemanticBlock};
use semantic_analysis::statements::StatementBlock;

pub struct MirInstructionGenerator {
    pub debug: bool,
}

pub struct MirInstructions {
    instructions: Vec<MirInstructionBlock>,
}

pub struct MirInstructionBlock {}

pub struct MirInstruction {
    pub label: Option<String>,
    pub result_operator: Option<MirOperator>,
    pub operand: MirInstructionOperand,
    pub first_arg_operator: Option<MirOperator>,
    pub second_arg_operator: Option<MirOperator>,
    pub flags: MirFlags,
}

impl MirInstruction {}

pub enum MirInstructionOperand {
    Add,
    Subtract,
    Multiply,
    Divide,
    Concat,
    CallFunction,
    PushStack,
    PopStack,
    JumpUnconditional,
    Label,
}

pub struct MirFlags {
    pub requires_cast: bool,
}

pub enum MirOperatorType {
    StringType,
    BooleanType,
    IntegerType,
    FloatType,
}

pub struct MirOperator {
    pub operator_type: MirOperatorType,
}

impl MirInstructionGenerator {
    pub fn generate_mir(&self, p: &Program) -> MirInstructions {
        let mut result = Vec::<MirInstructionBlock>::new();

        for block in p.get_blocks() {
            let mut block_mir = self.generate_block_mir(block);
            result.append(&mut block_mir);
        }

        MirInstructions {
            instructions: result,
        }
    }

    fn generate_block_mir(&self, s: SemanticBlock) -> Vec<MirInstructionBlock> {
        let mut result = Vec::<MirInstructionBlock>::new();

        let mut new_blocks = match s.get_block_type() {
            BlockType::AssignmentBlock => self.generate_assignment_mir(s.get_assignment_block()),
            BlockType::LoopBlock => self.generate_loop_mir(s.get_loop_block()),
            BlockType::FunctionBlock => self.generate_function_mir(s.get_function_block()),
            BlockType::BreakBlock => self.generate_break_mir(s.get_assignment_block()),
            BlockType::ReturnBlock => self.generate_return_mir(s.get_assignment_block()),
        };

        result.append(&mut new_blocks);
        result
    }

    fn generate_assignment_mir(&self, s: StatementBlock) -> Vec<MirInstructionBlock> {
        Vec::<MirInstructionBlock>::new()
    }

    fn generate_loop_mir(&self, l: LoopBlock) -> Vec<MirInstructionBlock> {
        Vec::<MirInstructionBlock>::new()
    }

    fn generate_function_mir(&self, f: FunctionBlock) -> Vec<MirInstructionBlock> {
        Vec::<MirInstructionBlock>::new()
    }

    fn generate_break_mir(&self, s: StatementBlock) -> Vec<MirInstructionBlock> {
        Vec::<MirInstructionBlock>::new()
    }

    fn generate_return_mir(&self, s: StatementBlock) -> Vec<MirInstructionBlock> {
        Vec::<MirInstructionBlock>::new()
    }

    fn generate_expression_mir(&self, s: ExprNode) -> Vec<MirInstructionBlock> {
        let mut result = Vec::<MirInstructionBlock>::new();

        let mut new_blocks = match s.get_expr_type() {

        };

        result.append(&mut new_blocks);
        result
    }
}
