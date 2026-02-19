use crate::Node;

impl Node {
    /// Get node by index
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
    ///
    /// let node_1 = Node::new_vec();
    /// let node_2 = Node::new();
    /// node_1.push_node(node_2);
    /// let node_2 = node_1.node_by_index(0);
    /// ```
    pub fn node_by_index(&self, i: usize) -> Option<Node> {
        self.body.read().unwrap().node_by_index(i)
    }

    /// Push node to vector node
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
    ///
    /// let node_1 = Node::new_vec();
    /// let node_2 = Node::new();
    /// node_1.push_node(node_2);
    /// ```
    pub fn push_node(&self, node: Node) {
        self.body.write().unwrap().push_node(node);
    }


    /// Pop node from vector node
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
    ///
    /// let node_1 = Node::new_vec();
    /// let node_2 = Node::new();
    /// node_1.push_node(node_2);
    /// let node_2 = node_1.pop_node();
    /// ```
    pub fn pop_node(&self) -> Option<Node> {
        self.body.write().unwrap().pop_node()
    }

    /// Remove node by index from vector node
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
    ///
    /// let node_1 = Node::new_vec();
    /// let node_2 = Node::new();
    /// node_1.push_node(node_2);
    /// let node_2 = node_1.remove_by_node_by_index(0);
    /// ```
    pub fn remove_by_node_by_index(&self, i: usize) -> Node {
        self.body.write().unwrap().remove_by_node_by_index(i)
    }

}

#[cfg(test)]
mod tests {
    use crate::Node;

    #[test]
    fn test_push_node_pop() {
        let node_1 = Node::new_vec();
        let node_2 = Node::new();

        node_1.push_node(node_2);
        let _node_2 = node_1.pop_node().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_push_node_panic() {
        let node_2 = Node::new();
        Node::new().push_node(node_2);
    }

    #[test]
    #[should_panic]
    fn test_push_pop_panic() {
        Node::new().pop_node();
    }

    #[test]
    fn test_node_by_index() {
        let node_1 = Node::new_vec();
        let node_2 = Node::new();

        node_1.push_node(node_2);
        let _node_2 = node_1.node_by_index(0).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_node_by_index_panic() {
        Node::new().node_by_index(0);
    }


    #[test]
    fn test_remove_node_by_index() {
        let node_1 = Node::new_vec();
        let node_2 = Node::new();

        node_1.push_node(node_2);
        let _node_2 = node_1.remove_by_node_by_index(0);
    }

    #[test]
    #[should_panic]
    fn test_remove_node_by_index_panic() {
        Node::new().remove_by_node_by_index(0);
    }
}