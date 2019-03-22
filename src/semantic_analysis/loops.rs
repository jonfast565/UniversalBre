use code_generation::visualizer::Visualizer;
use semantic_analysis::semantic_blocks::SemanticBlock;

#[derive(Debug, Clone, PartialEq)]
pub enum LoopType {
    InfiniteLoop,
    ConditionalLoop,
    ForLoop,
}

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
}

impl Visualizer for LoopBlock {
    fn build_graphviz(&self) -> String {
        String::new()
    }
}