use code_generation::visualizer::Visualizer;
use semantic_analysis::semantic_blocks::{BlockType, SemanticBlock};
use utilities::utility::Pair;

pub struct Program {
    blocks: Vec<SemanticBlock>,
}

impl Program {
    pub fn init(blocks: Vec<SemanticBlock>) -> Program {
        Program { blocks: blocks }
    }
    pub fn get_blocks(&self) -> Vec<SemanticBlock> {
        self.blocks.clone()
    }
}

impl Visualizer for Program {
    fn build_graphviz(&self) -> String {
        let blocks_ref = self.blocks.as_slice();
        let mut result = String::new();
        let mut last_id: Option<String> = None;
        let mut pairs = Vec::<Pair>::new();
        for block in blocks_ref {
            let id = match block {
                SemanticBlock {
                    block_type,
                    assignment_block: Some(block_ref),
                    ..
                } if *block_type == BlockType::AssignmentBlock => block_ref.id.to_string(),
                _ => block.id.to_string(),
            };
            result = format!(
                "{}\nsubgraph \"cluster_{}\" {{\n{}\n}}\n",
                result,
                id,
                block.build_graphviz()
            );
            match last_id {
                Some(last_id_match) => {
                    pairs.push(Pair(last_id_match, id.to_string()));
                    last_id = Some(id);
                }
                None => {
                    pairs.push(Pair("Program".to_string(), id.to_string()));
                    last_id = Some(id);
                }
            }
        }
        for pair in pairs {
            match pair {
                Pair(one, two) => {
                    result = format!("{}\n\"{}\" -> \"{}\"", result, one, two);
                }
            }
        }
        return format!("digraph g {{\nrankdir=\"LR\";\n{}\n}}\n", result);
    }
}
