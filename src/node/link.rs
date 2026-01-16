use crate::Node;

impl Node {
    /// Link one node to another
    ///
    /// # Example
    /// ```rust
    /// use heap_node::Node;
    ///
    /// let node_1 = Node::new();
    /// let node_2 = Node::new();
    /// node_2.link("node_1", node_1);
    /// ```
    pub fn link(&self, link_as: &'static str, node: Node) {
        self.body.write().unwrap().node_links.insert(link_as, node);
    }


    /// Unlink one node from another
    ///
    /// # Example
    /// ```rust
    /// use heap_node::Node;
    ///
    /// let node_1 = Node::new();
    /// let node_2 = Node::new();
    /// node_2.unlink("node_1");
    /// ```
    pub fn unlink(&self, k: &'static str) {
        self.body.write().unwrap().node_links.remove(k);
    }
}


#[cfg(test)]
mod tests {
    use crate::tests::tests::linked_nodes;

    #[test]
    fn test_link() {
        let node_2 = linked_nodes();
        let node_1 = node_2.node_by_chain(vec!["node_1"]);

        assert!(node_1.is_some());
    }

    #[test]
    fn test_unlink() {
        let node_2 = linked_nodes();
        node_2.unlink("node_1");

        let node_1 = node_2.node_by_chain(vec!["node_1"]);

        assert!(node_1.is_none());
    }
}