use semantic_analysis::data_types::DataType;
use semantic_analysis::expressions::{ExprNode, ExprType};
use semantic_analysis::functions::FunctionBlock;
use semantic_analysis::loops::LoopBlock;
use semantic_analysis::operation_types::OperationType;
use semantic_analysis::program::Program;
use semantic_analysis::semantic_blocks::{BlockType, SemanticBlock};
use semantic_analysis::statements::{AssignmentBlock, BreakBlock, ReturnBlock};

#[derive(Clone, PartialEq)]
pub struct MirInstructionGenerator {
    pub debug: bool,
}

#[derive(Clone, PartialEq)]
pub struct MirInstructions {
    blocks: Vec<MirInstructionBlock>,
}

#[derive(Clone, PartialEq)]
pub struct MirInstructionBlock {
    instructions: Vec<MirInstruction>,
}

impl MirInstructionBlock {
    pub fn empty_block() -> MirInstructionBlock {
        MirInstructionBlock {
            instructions: Vec::<MirInstruction>::new(),
        }
    }

    pub fn has_instructions(&self) -> bool {
        self.instructions.len() > 0
    }

    pub fn merge(&mut self, m: MirInstructionBlock) {
        let mut gotten_instructions = m.get_instructions();
        self.instructions.append(&mut gotten_instructions);
    }

    fn get_instructions(&self) -> Vec<MirInstruction> {
        return self.instructions.clone();
    }
}

#[derive(Clone, PartialEq)]
pub struct MirInstruction {
    pub label: Option<String>,
    pub result_operator: Option<MirOperator>,
    pub operand: OperationType,
    pub first_arg_operator: Option<MirOperator>,
    pub second_arg_operator: Option<MirOperator>,
    pub flags: MirFlags,
}

impl MirInstruction {}

#[derive(Clone, PartialEq)]
pub struct MirFlags {
    pub requires_cast: bool,
}

#[derive(Clone, PartialEq)]
pub struct MirOperator {
    pub operator_type: DataType,
    pub literal_value: Option<String>,
    pub variable_identifier: Option<String>,
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
        // get expression and generate
        self.generate_expression_mir(s.get_expression().unwrap())
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
        self.generate_expression_mir(s.return_expression.unwrap())
    }

    fn generate_expression_mir(&self, e: ExprNode) -> MirInstructionBlock {
        let mut instructions: Vec<MirInstruction> = Vec::new();
        MirInstructionGenerator::generate_expression_mir_internal(e, &mut instructions);
        MirInstructionBlock {
            instructions: instructions,
        }
    }

    fn generate_expression_mir_internal(e: ExprNode, instructions: &mut Vec<MirInstruction>) {
        let left_node = *(e.get_left_node().unwrap());
        let right_node = *(e.get_right_node().unwrap());

        if e.left_child_is_internal() && e.right_child_is_internal() {
            MirInstructionGenerator::generate_expression_mir_internal(left_node, instructions);
            MirInstructionGenerator::generate_expression_mir_internal(right_node, instructions);
        } else if e.left_child_is_internal() && !e.right_child_is_internal() {
            MirInstructionGenerator::generate_expression_mir_internal(left_node, instructions);
        } else if !e.left_child_is_internal() && e.right_child_is_internal() {
            MirInstructionGenerator::generate_expression_mir_internal(right_node, instructions);
        }

        let left_node_internal = *(e.get_left_node().unwrap());
        let right_node_internal = *(e.get_right_node().unwrap());

        instructions.push(MirInstructionGenerator::decode_internal(
            &e,
            &left_node_internal,
            &right_node_internal,
        ));
    }

    fn decode_internal(e: &ExprNode, left: &ExprNode, right: &ExprNode) -> MirInstruction {
        assert!(e.get_expression_type() == ExprType::Binary);
        MirInstruction {
            label: None,
            result_operator: Some(MirInstructionGenerator::decode_operand(e)),
            operand: e.get_operation_type(),
            first_arg_operator: Some(MirInstructionGenerator::decode_operand(left)),
            second_arg_operator: Some(MirInstructionGenerator::decode_operand(right)),
            flags: MirFlags {
                requires_cast: false,
            },
        }
    }

    fn decode_operand(e: &ExprNode) -> MirOperator {
        match e.get_expression_type() {
            ExprType::Literal => MirOperator {
                operator_type: e.get_type(),
                literal_value: Some(e.get_value()),
                variable_identifier: None,
            },
            _ => MirOperator {
                operator_type: e.get_type(),
                literal_value: None,
                variable_identifier: Some(e.get_id()),
            },
        }
    }
}
