pub trait Visualizer {
    fn build_graphviz(&self) -> String;
}

pub struct GraphvizFormatter {}

impl GraphvizFormatter {
    pub fn build_edge(node1: &String, node2: &String) -> String {
        format!("\"{}\" -> \"{}\"", node1, node2)
    }

    pub fn build_node_label(node: &String, label: &String) -> String {
        format!("\"{}\" [label=\"{}\"]", node, label)
    }

    pub fn build_node_bilabel(node: &String, label1: &String, label2: &String) -> String {
        format!("\"{}\" [label=\"{}: {}\"]", node, label1, label2)
    }

    pub fn build_binary_node(
        node: &String,
        label: &String,
        first_edges: &String,
        second_edges: &String,
    ) -> String {
        format!(
            "{}\n{}\n{}",
            GraphvizFormatter::build_node_label(node, label),
            first_edges,
            second_edges
        )
    }

    pub fn concat_three(one: &String, two: &String, three: &String) -> String {
        format!("{}\n{}\n{}", one, two, three)
    }
}
