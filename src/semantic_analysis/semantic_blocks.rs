use code_generation::visualizer::Visualizer;
use semantic_analysis::functions::FunctionBlock;
use semantic_analysis::loops::LoopBlock;
use semantic_analysis::statements::StatementBlock;
use utilities::utility;

#[derive(Debug, Clone, PartialEq)]
pub enum BlockType {
    StatementBlock,
    LoopBlock,
    FunctionBlock,
    BreakBlock,
}

pub struct SemanticBlock {
    pub id: String,
    pub block_type: BlockType,
    pub statement_block: Option<StatementBlock>,
    loop_block: Option<LoopBlock>,
    function_block: Option<FunctionBlock>,
}

impl SemanticBlock {
    pub fn init_with_statement(statement_block: StatementBlock) -> SemanticBlock {
        SemanticBlock {
            id: utility::get_new_uuid(),
            block_type: BlockType::StatementBlock,
            statement_block: Some(statement_block),
            loop_block: None,
            function_block: None,
        }
    }
    pub fn init_with_loop(loop_block: LoopBlock) -> SemanticBlock {
        SemanticBlock {
            id: utility::get_new_uuid(),
            block_type: BlockType::LoopBlock,
            statement_block: None,
            loop_block: Some(loop_block),
            function_block: None,
        }
    }
    pub fn init_with_function(function_block: FunctionBlock) -> SemanticBlock {
        SemanticBlock {
            id: utility::get_new_uuid(),
            block_type: BlockType::FunctionBlock,
            statement_block: None,
            loop_block: None,
            function_block: Some(function_block),
        }
    }
    pub fn init_with_break() -> SemanticBlock {
        SemanticBlock {
            id: utility::get_new_uuid(),
            block_type: BlockType::BreakBlock,
            statement_block: None,
            loop_block: None,
            function_block: None,
        }
    }
}

impl Visualizer for SemanticBlock {
    fn build_graphviz(&self) -> String {
        if let Some(statement_block) = self.statement_block.as_ref() {
            return statement_block.build_graphviz();
        } else if let Some(loop_block) = self.loop_block.as_ref() {
            return loop_block.build_graphviz();
        } else if let Some(function_block) = self.function_block.as_ref() {
            return function_block.build_graphviz();
        }
        panic!("This should never happen");
    }
}
