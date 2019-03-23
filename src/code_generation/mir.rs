use semantic_analysis::expressions::{ExprNode, ExprType};
use semantic_analysis::functions::FunctionBlock;
use semantic_analysis::loops::LoopBlock;
use semantic_analysis::program::Program;
use semantic_analysis::semantic_blocks::{BlockType, SemanticBlock};
use semantic_analysis::statements::{AssignmentBlock, BreakBlock, ReturnBlock};

pub struct MirInstructionGenerator {
    pub debug: bool,
}

pub struct MirInstructions {
    blocks: Vec<MirInstructionBlock>,
}

pub struct MirInstructionBlock {
    instructions: Vec<MirInstruction>,
}

impl MirInstructionBlock {
    pub fn has_instructions(&self) -> bool {
        self.instructions.len() > 0
    }
}

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

        MirInstructions { blocks: result }
    }

    fn generate_block_mir(&self, s: SemanticBlock) -> Vec<MirInstructionBlock> {
        let mut result = Vec::<MirInstructionBlock>::new();

        let new_block = match s.get_block_type() {
            BlockType::AssignmentBlock => self.generate_assignment_mir(s.get_assignment_block()),
            BlockType::LoopBlock => self.generate_loop_mir(s.get_loop_block()),
            BlockType::FunctionBlock => self.generate_function_mir(s.get_function_block()),
            BlockType::BreakBlock => self.generate_break_mir(s.get_break_block()),
            BlockType::ReturnBlock => self.generate_return_mir(s.get_return_block()),
        };

        result.push(new_block);
        result
    }

    fn generate_assignment_mir(&self, s: AssignmentBlock) -> MirInstructionBlock {
        let instructions = Vec::<MirInstruction>::new();

        MirInstructionBlock {
            instructions: instructions,
        }
    }

    fn generate_loop_mir(&self, l: LoopBlock) -> MirInstructionBlock {
        let instructions = Vec::<MirInstruction>::new();

        MirInstructionBlock {
            instructions: instructions,
        }
    }

    fn generate_function_mir(&self, f: FunctionBlock) -> MirInstructionBlock {
        let instructions = Vec::<MirInstruction>::new();

        MirInstructionBlock {
            instructions: instructions,
        }
    }

    fn generate_break_mir(&self, s: BreakBlock) -> MirInstructionBlock {
        let instructions = Vec::<MirInstruction>::new();

        MirInstructionBlock {
            instructions: instructions,
        }
    }

    fn generate_return_mir(&self, s: ReturnBlock) -> MirInstructionBlock {
        let instructions = Vec::<MirInstruction>::new();

        MirInstructionBlock {
            instructions: instructions,
        }
    }

    fn generate_expression_mir(&self, s: ExprNode) -> MirInstructionBlock {
        match s.get_expression_type() {
            ExprType::Binary => {
                let mut result = Vec::<MirInstruction>::new();

                MirInstructionBlock {
                    instructions: result,
                }
            }
            _ => MirInstructionBlock {
                instructions: Vec::<MirInstruction>::new(),
            },
        }
    }
}
