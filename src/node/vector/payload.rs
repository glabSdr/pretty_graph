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
    pub fn push_str(&self, v: &str) {
        self.body.write().unwrap().push_str(v);
    }

    /// Get node value
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
    ///
    /// let node = Node::new();
    /// node.push_str("value");
    /// let v = node.get_string_by_index(0);
    /// ```
    pub fn get_string_by_index(&self, i: usize) -> Option<String> {
        self.body.read().unwrap().get_string_by_index(i)
    }

    /// Remove node value
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
    ///
    /// let node = Node::new();
    /// node.push_str("value");
    /// let v = node.remove_string_by_index(0);
    /// ```
    pub fn remove_string_by_index(&self, i: usize) -> String {
        self.body.write().unwrap().remove_string_by_index(i)
    }


    /// Check if vector node contains string
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
    ///
    /// let node = Node::new();
    /// node.push_str("value");
    /// let v = node.contains_string(&"value".to_string());
    /// ```
    pub fn contains_string(&self, v: &String) -> bool {
        self.body.read().unwrap().contains_string(v)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_get() {
        let node = Node::new_vec();
        node.push_str("value");

        let v = node.get_string_by_index(0).unwrap();
        assert_eq!(v, "value");
    }

    #[test]
    fn test_remove_string_by_index() {
        let node = Node::new_vec();
        node.push_str("value");

        let v = node.remove_string_by_index(0);
        assert_eq!(v, "value");
    }

    #[test]
    fn test_contains_string() {
        let node = Node::new_vec();
        node.push_str("value");
        let v = node.contains_string(&"value".to_string());
        assert!(v);
    }

    #[test]
    #[should_panic]
    fn test_push_str_panic() {
        Node::new().push_str("value");
    }

    #[test]
    #[should_panic]
    fn test_get_string_by_index_panic() {
        Node::new().get_string_by_index(0);
    }

    #[test]
    #[should_panic]
    fn test_remove_string_by_index_panic() {
        Node::new().remove_string_by_index(0);
    }

    #[test]
    #[should_panic]
    fn test_empty_remove_string_by_index_panic() {
        let node = Node::new_vec();

        let _ = node.remove_string_by_index(0);
    }

    #[test]
    #[should_panic]
    fn test_contains_string_panic() {
        Node::new().contains_string(&"value".to_string());
    }
}