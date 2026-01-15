use crate::Node;

impl Node {

    /// Set node value
    /// # Example
    /// ```rust
    /// use heap_node::Node;
    ///
    /// let node = Node::new();
    /// node.set("key", "value");
    /// ```
    pub fn set(&self, k: &'static str, v: &'static str) {
        self.body.borrow_mut().payload.insert(k, v);
    }

    /// Get node value
    /// # Example
    /// ```rust
    /// use heap_node::Node;
    ///
    /// let node = Node::new();
    /// node.set("key", "value");
    /// let v = node.get("key");
    /// ```
    pub fn get(&self, k: &str) -> Option<&str> {
        self.body.borrow().payload.get(k).cloned()
    }

    /// Remove node value
    /// # Example
    /// ```rust
    /// use heap_node::Node;
    ///
    /// let node = Node::new();
    /// node.set("key", "value");
    /// let v = node.remove("key");
    /// ```
    pub fn remove(&self, k: &str) -> Option<&str> {
        self.body.borrow_mut().payload.remove(k)
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
}