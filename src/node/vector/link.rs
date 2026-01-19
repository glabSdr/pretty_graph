use crate::Node;

impl Node {
    /// Set node value
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
    ///
    /// let node = Node::new();
    /// node.push_str("value");
    /// ```
    pub fn push_node(&self, node: Node) {
        self.body.write().unwrap().push_node(node);
    }
}


#[cfg(test)]
mod tests {
    use crate::Node;

    #[test]
    fn test_push_node() {
        let node_1 = Node::new_vec();
        let node_2 = Node::new();

        node_1.push_node(node_2);
    }

    #[test]
    #[should_panic]
    fn test_push_node_panic() {
        let node_2 = Node::new();
        Node::new().push_node(node_2);
    }
}