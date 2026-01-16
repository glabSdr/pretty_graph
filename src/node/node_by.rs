use crate::Node;

impl Node {
    /// Get to node by link chane
    ///
    /// # Example
    /// ```rust
    /// use heap_node::Node;
    ///
    /// let node_1 = Node::from(None, None);
    /// let node_2 = Node::from(
    ///     None,
    ///     Some(vec![
    ///         ("node_1", node_1.clone())
    ///     ])
    /// );
    ///
    ///
    /// let node_1: Option<Node> = node_2.node_by_key("node_1");
    /// ```
    pub fn node_by_key(&self, k: &str) -> Option<Node> {
        self.body.read().unwrap().node_links.get(k).cloned()
    }

    /// Get to node by link chane
    ///
    /// # Example
    /// ```rust
    /// use heap_node::Node;
    ///
    /// let node_1 = Node::from(None, None);
    /// let node_2 = Node::from(
    ///     None,
    ///     Some(vec![
    ///         ("node_1", node_1.clone())
    ///     ])
    /// );
    ///
    /// let node_3 = Node::from(
    ///     None,
    ///     Some(vec![
    ///         ("node_2", node_2.clone())
    ///     ])
    /// );
    ///
    /// let node_1: Option<Node> = node_3.node_by_chain(vec!["node_2", "node_1"]);
    /// ```
    pub fn node_by_chain(&self, mut route: Vec<&str>) -> Option<Node> {
        let k = route.remove(0);
        let binding = self.body.read().unwrap();
        let next = match binding.node_links.get(k) {
            Some(node) => node,
            None => return None
        };

        if route.len() == 0 {
            return Some(next.clone());
        };

        next.node_by_chain(route)
    }
}



#[cfg(test)]
mod tests {
    use crate::tests::tests::linked_nodes;
    use super::*;

    #[test]
    fn test_node_by_key() {
        let node_2 = linked_nodes();

        let node_1: Option<Node>  = node_2.node_by_key("node_1");

        assert!(node_1.is_some());
    }

    #[test]
    fn test_node_by_chain() {
        let node_2 = linked_nodes();
        let node_3 = Node::new();
        node_3.link("node_2", node_2.clone());

        let node_1: Option<Node>  = node_3.node_by_chain(vec!["node_2", "node_1"]);

        assert!(node_1.is_some());
    }
}