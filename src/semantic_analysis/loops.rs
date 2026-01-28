use code_generation::visualizer::Visualizer;
use semantic_analysis::semantic_blocks::SemanticBlock;

#[derive(Debug, Clone, PartialEq)]
pub enum LoopType {
    InfiniteLoop,
    // ConditionalLoop,
    // ForLoop,
}

#[derive(Clone, PartialEq, Debug)]
pub struct LoopBlock {
    loop_type: LoopType,
    statement_list: Vec<SemanticBlock>,
}

impl LoopBlock {
    pub fn init(loop_type: LoopType, statement_list: Vec<SemanticBlock>) -> LoopBlock {
        LoopBlock {
            loop_type: loop_type,
            statement_list: statement_list,
        }
    }

    pub fn get_loop_type(&self) -> LoopType {
        self.loop_type.clone()
    }

    pub fn get_statement_list(&self) -> Vec<SemanticBlock> {
        self.statement_list.clone()
    }
}

impl Visualizer for LoopBlock {
    fn build_graphviz(&self) -> String {
        String::new()
    }
}
