use code_generation::visualizer::Visualizer;

#[derive(Debug, Clone, PartialEq)]
pub enum LoopType {
    InfiniteLoop,
    ConditionalLoop,
    ForLoop,
}

pub struct LoopBlock {
    loop_type: LoopType,
}

impl LoopBlock {
    pub fn init(loop_type: LoopType) -> LoopBlock {
        LoopBlock {
            loop_type: loop_type,
        }
    }
}

impl Visualizer for LoopBlock {
    fn build_graphviz(&self) -> String {
        String::new()
    }
}