use crate::Node;

impl Node {
    /// Get payload values
    ///
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
    ///
    ///let node = Node::new();
    ///node.set("key1", "value1");
    ///node.set("key2", "value2");
    ///
    ///let values = vec!["value1", "value2"];
    ///
    ///for v in node.values().iter() {
    ///    assert!(values.contains(&v.as_str()));
    ///}
    /// ```
    pub fn values(&self) -> Vec<String> {
        self.body.read().unwrap().values()
    }



    /// Clear payload
    ///
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
    ///
    ///let node = Node::new();
    ///
    ///node.clear();
    /// ```
    pub fn clear(&self) {
        self.body.write().unwrap().clear();
    }


    /// Get payload len
    /// # Example
    /// ```rust
    ///use pretty_graph::Node;
    ///
    ///let node = Node::new_vec();
    ///node.push_str("Hi!");
    ///
    ///println!("len: {}", node.len());
    ///
    ///let node = Node::new();
    ///node.set("msg", "Hi!");
    ///
    ///println!("len: {}", node.len());
    ///
    /// ```
    pub fn len(&self) -> usize {
        self.body.read().unwrap().len()
    }

    /// Check if vector node contains string
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
    ///
    /// let node = Node::new();
    /// node.set("key", "value");
    /// let v = node.contains_string(&"value".to_string());
    ///
    /// let node = Node::new_vec();
    /// node.push_str("value");
    /// let v = node.contains_string(&"value".to_string());
    /// ```
    pub fn contains_string(&self, v: &String) -> bool {
        self.body.read().unwrap().contains_string(v)
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

    #[test]
    pub fn test_map_clear() {
        let node = Node::vec_from(Some(vec!["value1"]), None);
        node.clear();

        let values = node.values();
        assert_eq!(values.len(), 0);
    }

    #[test]
    pub fn test_vec_clear() {
        let node = Node::from(Some(vec![("key1","value1")]), None);
        node.clear();

        let values = node.values();
        assert_eq!(values.len(), 0);
    }

    #[test]
    pub fn test_len() {
        let node = Node::new_vec();
        node.push_str("Hi!");

        assert_eq!(node.len(), 1);

        let node = Node::new();
        node.set("msg", "Hi!");

        assert_eq!(node.len(), 1);
    }

    #[test]
    fn test_contains_string() {
        let node = Node::new_vec();
        node.push_str("value");
        let v = node.contains_string(&"value".to_string());
        assert!(v);

        let node = Node::new();
        node.set("key1", "value1");
        node.set("key2", "value2");
        let v = node.contains_string(&"value2".to_string());
        assert!(v);

        let node = Node::new();
        let v = node.contains_string(&"value".to_string());
        assert!(!v);
    }
}