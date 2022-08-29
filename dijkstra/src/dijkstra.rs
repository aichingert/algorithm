use crate::node::Node;

pub struct Dijkstra {
    pub current_node_value: String,
    pub nodes: Vec<Node>
}

impl Dijkstra {
    pub fn solve(&self) -> Vec<Node> {

        vec![]
    }

    pub fn default() -> Self {
        Self {
            current_node_value: "".to_string(),
            nodes: vec![]
        }
    }
}

impl std::fmt::Debug for Dijkstra {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: String = String::new();

        for i in 0..self.nodes.len() {
            output.push_str(&self.nodes[i].to_string());
        }
        write!(f, "{output}")
    }
}