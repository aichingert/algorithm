pub struct Node {
    value: i32,
    connections: Vec<Option<Box<Node>>>
}

impl Node {
    fn new(value: i32, connections: Vec<Option<Box<Node>>>) -> Self {
        Self {
            value,
            connections
        }
    }
}