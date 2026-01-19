use crate::Node;

impl Node {

    /// Set node value
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
    ///
    /// let node = Node::new();
    /// node.set("key", "value");
    /// ```
    pub fn set(&self, k: &str, v: &str) {
        self.body.write().unwrap().set(k, v);
    }

    /// Get node value
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
    ///
    /// let node = Node::new();
    /// node.set("key", "value");
    /// let v = node.get("key");
    /// ```
    pub fn get(&self, k: &str) -> Option<String> {
        self.body.read().unwrap().get(k)
    }

    /// Remove node value
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
    ///
    /// let node = Node::new();
    /// node.set("key", "value");
    /// let v = node.remove("key");
    /// ```
    pub fn remove(&self, k: &str) -> Option<String> {
        self.body.write().unwrap().remove(k)
    }

    /// Get payload keys
    ///
    /// # Example
    /// ```rust
    ///let node = Node::new();
    ///node.set("key1", "value1");
    ///node.set("key2", "value2");
    ///
    ///let keys = vec!["key1", "key2"];
    ///
    ///for k in node.keys() {
    ///    assert!(keys.contains(&k));
    ///}
    /// ```
    pub fn keys(&self) -> Vec<String> {
        self.body.read().unwrap().keys()
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get_remove() {
        let node = Node::new();
        node.set("key", "value");

        let v = node.get("key").unwrap();
        assert_eq!(v, "value");

        node.remove("key");
        assert!(node.get("key").is_none());
    }

    #[test]
    fn test_keys() {
        let node = Node::new();
        node.set("key1", "value1");
        node.set("key2", "value2");

        let keys = vec!["key1", "key2"];

        for k in node.keys() {
            assert!(keys.contains(&k.as_str()));
        }
    }

    #[test]
    fn test_values() {
        let node = Node::new();
        node.set("key1", "value1");
        node.set("key2", "value2");

        let values = vec!["value1", "value2"];

        for v in node.values() {
            assert!(values.contains(&v.as_str()));
        }
    }

    #[test]
    #[should_panic]
    fn test_unlink_panic() {
        Node::new_vec().set("key", "value");
    }

    #[test]
    #[should_panic]
    fn test_link_keys_panic() {
        Node::new_vec().get("key");
    }

    #[test]
    #[should_panic]
    fn test_node_by_key_panic() {
        Node::new_vec().remove("key");
    }

    #[test]
    #[should_panic]
    fn test_keys_panic() {
        Node::new_vec().keys();;
    }
}