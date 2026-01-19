use crate::Node;

impl Node {
    /// Link one node to another
    ///
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
    ///
    /// let node_1 = Node::new();
    /// let node_2 = Node::new();
    /// node_2.link("node_1", node_1);
    /// ```
    pub fn link(&self, link_as: &str, node: Node) {
        self.body.write().unwrap().link(link_as, node);
    }


    /// Unlink one node from another
    ///
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
    ///
    /// let node_1 = Node::new();
    /// let node_2 = Node::new();
    /// node_2.unlink("node_1");
    /// ```
    pub fn unlink(&self, k: &str) {
        self.body.write().unwrap().unlink(k);
    }

    /// Get linked node keys
    ///
    /// # Example
    /// ```rust
    ///let node_3 = Node::new();
    ///let node_2 = Node::new();
    ///let node_1 = Node::new();
    ///
    ///node_1.link("node_2", node_2);
    ///node_1.link("node_3", node_3);
    ///
    ///for k in node_1.link_keys() {
    ///    // ...
    ///}
    /// ```
    pub fn link_keys(&self) -> Vec<String> {
        self.body.read().unwrap().link_keys()
    }

    /// Get to node by link chane
    ///
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
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
        self.body.read().unwrap().node_by_key(k)
    }

    /// Get to node by link chane
    ///
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
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
        let next = match self.node_by_key(k) {
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
    use crate::Node;
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

    #[test]
    fn test_link_keys() {
        let node_3 = Node::new();
        let node_2 = linked_nodes();
        node_2.link("node_3", node_3);

        let link_names = vec!["node_1", "node_3"];

        for k in node_2.link_keys() {
            assert!(link_names.contains(&k.as_str()));
        }
    }

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

    #[test]
    #[should_panic]
    fn test_link_panic() {
        let node = Node::new();
        Node::new_vec().link("key", node);
    }

    #[test]
    #[should_panic]
    fn test_unlink_panic() {
        Node::new_vec().unlink("key");
    }

    #[test]
    #[should_panic]
    fn test_link_keys_panic() {
        Node::new_vec().link_keys();
    }

    #[test]
    #[should_panic]
    fn test_node_by_key_panic() {
        Node::new_vec().node_by_key("key");
    }

}