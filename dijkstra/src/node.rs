pub struct Node {
    value: i32,
    connections: Vec<Option<Box<Node>>>
}

impl Node {
    pub fn new(value: i32, connections: Vec<Option<Box<Node>>>) -> Self {
        Self {
            value,
            connections
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node: {} - {:?} \n", self.value, self.connections)
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node: {} - {:?} \n", self.value, self.connections)
    }
}