pub trait Visualizer {
    fn build_graphviz(&self) -> String;
}
