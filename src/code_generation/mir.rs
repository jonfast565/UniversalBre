use semantic_analysis::data_types::DataType;
use semantic_analysis::expressions::{ExprNode, ExprType};
use semantic_analysis::functions::FunctionBlock;
use semantic_analysis::loops::LoopBlock;
use semantic_analysis::operation_types::OperationType;
use semantic_analysis::program::Program;
use semantic_analysis::semantic_blocks::{BlockType, SemanticBlock};
use semantic_analysis::statements::{AssignmentBlock, BreakBlock, ReturnBlock};

use std::fmt;
use std::sync::Arc;

#[derive(Clone, PartialEq, Debug)]
pub struct MirInstructionGenerator {
    pub debug: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MirInstructions {
    blocks: Vec<MirInstructionBlock>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MirVariableTarget {
    assignment_var_id: String,
    assignment_var_name: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MirInstructionBlock {
    assignment_variable: Option<MirVariableTarget>,
    instructions: Vec<MirInstruction>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MirInstruction {
    pub label: Option<String>,
    pub result_operator: Option<MirOperator>,
    pub operand: OperationType,
    pub first_arg_operator: Option<MirOperator>,
    pub second_arg_operator: Option<MirOperator>,
    pub flags: MirFlags,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MirFlags {
    pub requires_cast: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MirOperator {
    pub operator_type: DataType,
    pub literal_value: Option<String>,
    pub variable_identifier: Option<String>,
}

impl fmt::Display for MirOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(value) = &self.literal_value {
            return write!(f, "{}{}", self.operator_type.to_string(), value);
        }
        write!(
            f,
            "{}{}",
            self.operator_type.to_string(),
            self.variable_identifier.as_ref().unwrap()
        )
    }
}

impl MirInstructionGenerator {
    pub fn generate_mir(&self, p: &Program) -> MirInstructions {
        let mut result = Vec::<MirInstructionBlock>::new();

        for block in p.get_blocks() {
            let mut block_mir = self.generate_block_mir(block);
            result.append(&mut block_mir);
        }

        let instructions = MirInstructions { blocks: result };
        dbg!(instructions.clone());
        instructions
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
        let instructions = self.generate_expression_mir(s.get_expression().unwrap());
        let assign_block_copy = s.clone();
        MirInstructionBlock {
            assignment_variable: Some(MirVariableTarget {
                assignment_var_id: s.id,
                assignment_var_name: assign_block_copy.get_assignment_id(),
            }),
            instructions: instructions,
        }
    }

    fn generate_loop_mir(&self, _l: LoopBlock) -> MirInstructionBlock {
        let instructions = Vec::<MirInstruction>::new();
        MirInstructionBlock {
            assignment_variable: None,
            instructions: instructions,
        }
    }

    fn generate_function_mir(&self, _f: FunctionBlock) -> MirInstructionBlock {
        let instructions = Vec::<MirInstruction>::new();
        MirInstructionBlock {
            assignment_variable: None,
            instructions: instructions,
        }
    }

    fn generate_break_mir(&self, _s: BreakBlock) -> MirInstructionBlock {
        let instructions = Vec::<MirInstruction>::new();
        MirInstructionBlock {
            assignment_variable: None,
            instructions: instructions,
        }
    }

    fn generate_return_mir(&self, s: ReturnBlock) -> MirInstructionBlock {
        let instructions = self.generate_expression_mir(s.return_expression.unwrap());
        MirInstructionBlock {
            assignment_variable: None,
            instructions: instructions,
        }
    }

    fn generate_expression_mir(&self, e: Arc<ExprNode>) -> Vec<MirInstruction> {
        let instructions = self.generate_expression_mir_internal(e);
        instructions
    }

    fn generate_expression_mir_internal(&self, e: Arc<ExprNode>) -> Vec<MirInstruction> {
        let mut instructions = Vec::<MirInstruction>::new();
        if !e.is_internal() {
            return instructions;
        }
        
        let left_node = e.get_left_node().unwrap();
        let right_node = e.get_right_node().unwrap();

        if e.left_child_is_internal() {
            let left_node_int = e.get_left_node().unwrap();
            let mut left_instructions = self.generate_expression_mir_internal(left_node_int);
            instructions.append(&mut left_instructions);
        }

        if e.right_child_is_internal() {
            let right_node_int = e.get_right_node().unwrap();
            let mut right_instructions = self.generate_expression_mir_internal(right_node_int);
            instructions.append(&mut right_instructions);
        }

        let internal_instruction = self.decode_internal(e, left_node, right_node);
        instructions.push(internal_instruction);
        instructions
    }

    fn decode_internal(
        &self,
        e: Arc<ExprNode>,
        left: Arc<ExprNode>,
        right: Arc<ExprNode>,
    ) -> MirInstruction {
        assert!(e.get_expression_type() == &ExprType::Binary);
        let operation_type = *e.get_operation_type();
        let result_instruction = MirInstruction {
            label: None,
            result_operator: Some(self.decode_operand(e)),
            operand: operation_type,
            first_arg_operator: Some(self.decode_operand(left)),
            second_arg_operator: Some(self.decode_operand(right)),
            flags: MirFlags {
                requires_cast: false,
            },
        };

        result_instruction
    }

    fn decode_operand(&self, e: Arc<ExprNode>) -> MirOperator {
        match e.get_expression_type() {
            ExprType::Literal => MirOperator {
                operator_type: *e.get_type(),
                literal_value: Some(e.get_value()),
                variable_identifier: None,
            },
            _ => MirOperator {
                operator_type: *e.get_type(),
                literal_value: None,
                variable_identifier: Some(e.get_id()),
            },
        }
    }
}
