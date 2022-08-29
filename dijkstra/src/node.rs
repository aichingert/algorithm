pub struct Node {
    pub id: u16,
    pub value: i32,
    pub connections: Vec<Option<Box<Node>>>
}

impl Node {
    pub fn new(id: u16, value: i32, connections: Vec<Option<Box<Node>>>) -> Self {
        Self {
            id,
            value,
            connections
        }
    }

    pub fn default() -> Self {
        Self {
            id: 0,
            value: 0,
            connections: vec![]
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ id: {} - value: {} - connections: {:?} }}", self.id, self.value, self.connections)
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ id: {} - value: {} - connections: {:?} }}", self.id, self.value, self.connections)
    }
}