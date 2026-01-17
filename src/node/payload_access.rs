use crate::Node;

impl Node {
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
        self.body.read().unwrap().payload.keys().cloned().collect()
    }

    /// Get payload values
    ///
    /// # Example
    /// ```rust
    ///let node = Node::new();
    ///node.set("key1", "value1");
    ///node.set("key2", "value2");
    ///
    ///let values = vec!["value1", "value2"];
    ///
    ///for v in node.values() {
    ///    assert!(values.contains(&v));
    ///}
    /// ```
    pub fn values(&self) -> Vec<String> {
        self.body.read().unwrap().payload.values().cloned().collect()
    }
}


#[cfg(test)]
mod tests {
    use crate::tests::tests::linked_nodes;
    use super::*;

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
}
