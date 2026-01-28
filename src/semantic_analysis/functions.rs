use code_generation::visualizer::Visualizer;
use semantic_analysis::semantic_blocks::SemanticBlock;

#[derive(Clone, PartialEq, Debug)]
pub struct ArgumentBlock {
    name: String,
}

impl ArgumentBlock {
    pub fn init(name: String) -> ArgumentBlock {
        ArgumentBlock { name: name }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Clone, PartialEq, Debug)]
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

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_arguments(&self) -> Vec<ArgumentBlock> {
        self.arguments.clone()
    }

    pub fn get_body(&self) -> Vec<SemanticBlock> {
        self.body.clone()
    }
}

impl Visualizer for FunctionBlock {
    fn build_graphviz(&self) -> String {
        String::new()
    }
}
