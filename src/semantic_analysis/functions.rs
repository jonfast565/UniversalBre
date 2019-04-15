use code_generation::visualizer::Visualizer;
use semantic_analysis::semantic_blocks::SemanticBlock;

#[derive(Clone, PartialEq)]
pub struct ArgumentBlock {
    name: String,
}

impl ArgumentBlock {
    pub fn init(name: String) -> ArgumentBlock {
        ArgumentBlock { name: name }
    }
}

#[derive(Clone, PartialEq)]
pub struct FunctionBlock {
    name: String,
    arguments: Vec<ArgumentBlock>,
    body: Vec<SemanticBlock>,
}

impl FunctionBlock {
    pub fn init(
        name: String,
        arguments: Vec<ArgumentBlock>,
        body: Vec<SemanticBlock>,
    ) -> FunctionBlock {
        FunctionBlock {
            name: name,
            arguments: arguments,
            body: body,
        }
    }
}

impl Visualizer for FunctionBlock {
    fn build_graphviz(&self) -> String {
        String::new()
    }
}
