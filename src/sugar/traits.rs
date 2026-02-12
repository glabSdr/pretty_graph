
/// Universal wrappers on top of Node.
/// Works for both map and vector type nodes.
pub trait Universal {
    /// Get node by key and then trys to return its values or vector you set into or argument.
    /// Example
    /// ```rust
    /// use pretty_graph::Node;
    /// let node_1 = Node::new();
    /// let node_2 = Node::new_vec();
    /// node_2.push_str("Hello!");
    /// node_1.link("node_2", node_2);
    ///
    /// let vector = node_1.get_str_vector_by_key_or("node_2", vec![])
    ///
    /// ```
    fn get_str_vector_by_key_or(&self, k: &str, or: Vec<&str>) -> Vec<String>;
}