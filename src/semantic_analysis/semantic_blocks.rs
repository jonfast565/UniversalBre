use code_generation::visualizer::Visualizer;
use semantic_analysis::functions::FunctionBlock;
use semantic_analysis::loops::LoopBlock;
use semantic_analysis::statements::{AssignmentBlock, BreakBlock, ReturnBlock};
use utilities::utility;

#[derive(Debug, Clone, PartialEq)]
pub enum BlockType {
    AssignmentBlock,
    LoopBlock,
    FunctionBlock,
    BreakBlock,
    ReturnBlock,
}

#[derive(Clone, PartialEq)]
pub struct SemanticBlock {
    pub id: String,
    pub block_type: BlockType,
    pub assignment_block: Option<AssignmentBlock>,
    pub loop_block: Option<LoopBlock>,
    pub function_block: Option<FunctionBlock>,
    pub return_block: Option<ReturnBlock>,
    pub break_block: Option<BreakBlock>,
}

impl SemanticBlock {
    pub fn init_assignment(assignment_block: AssignmentBlock) -> SemanticBlock {
        SemanticBlock {
            id: utility::get_new_uuid(),
            block_type: BlockType::AssignmentBlock,
            assignment_block: Some(assignment_block),
            loop_block: None,
            function_block: None,
            break_block: None,
            return_block: None,
        }
    }

    pub fn init_loop(loop_block: LoopBlock) -> SemanticBlock {
        SemanticBlock {
            id: utility::get_new_uuid(),
            block_type: BlockType::LoopBlock,
            assignment_block: None,
            loop_block: Some(loop_block),
            function_block: None,
            break_block: None,
            return_block: None,
        }
    }

    pub fn init_function(function_block: FunctionBlock) -> SemanticBlock {
        SemanticBlock {
            id: utility::get_new_uuid(),
            block_type: BlockType::FunctionBlock,
            assignment_block: None,
            loop_block: None,
            function_block: Some(function_block),
            break_block: None,
            return_block: None,
        }
    }

    pub fn init_break(break_block: BreakBlock) -> SemanticBlock {
        SemanticBlock {
            id: utility::get_new_uuid(),
            block_type: BlockType::BreakBlock,
            assignment_block: None,
            loop_block: None,
            function_block: None,
            break_block: Some(break_block),
            return_block: None,
        }
    }

    pub fn init_return(return_block: ReturnBlock) -> SemanticBlock {
        SemanticBlock {
            id: utility::get_new_uuid(),
            block_type: BlockType::ReturnBlock,
            assignment_block: None,
            loop_block: None,
            function_block: None,
            break_block: None,
            return_block: Some(return_block),
        }
    }

    pub fn get_block_type(&self) -> BlockType {
        self.block_type.clone()
    }

    pub fn get_assignment_block(&self) -> AssignmentBlock {
        self.clone().assignment_block.unwrap()
    }

    pub fn get_function_block(&self) -> FunctionBlock {
        self.clone().function_block.unwrap()
    }

    pub fn get_loop_block(&self) -> LoopBlock {
        self.clone().loop_block.unwrap()
    }

    pub fn get_return_block(&self) -> ReturnBlock {
        self.clone().return_block.unwrap()
    }

    pub fn get_break_block(&self) -> BreakBlock {
        self.clone().break_block.unwrap()
    }
}

impl Visualizer for SemanticBlock {
    fn build_graphviz(&self) -> String {
        if let Some(assignment_block) = self.assignment_block.as_ref() {
            return assignment_block.build_graphviz();
        } else if let Some(loop_block) = self.loop_block.as_ref() {
            return loop_block.build_graphviz();
        } else if let Some(function_block) = self.function_block.as_ref() {
            return function_block.build_graphviz();
        }
        panic!("This should never happen");
    }
}
