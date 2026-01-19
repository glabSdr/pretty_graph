use crate::Node;

impl Node {
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
        self.body.read().unwrap().values()
    }
}

#[cfg(test)]
mod tests {
    use crate::Node;

    #[test]
    pub fn test_values() {
        let node = Node::vec_from(Some(vec!["value1", "value2"]), None);
        node.values();
        let values = node.values();
        for value in values {
            assert!(value == "value1".to_string() || value == "value2".to_string());
        }
    }
}