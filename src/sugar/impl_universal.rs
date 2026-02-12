use crate::sugar::traits::Universal;
use crate::Node;

impl Universal for Node {
    fn get_str_vector_by_key_or(&self, k: &str, or: Vec<&str>) -> Vec<String> {
        match self.node_by_key(k) {
            Some(node) => node.values(),
            None => or.iter().map(|v| String::from(*v)).collect()
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::Node;
    use crate::sugar::traits::Universal;

    #[test]
    fn test_get_str_vector_by_key_or() {
        let node_1 = Node::new();
        let node_2 = Node::new_vec();
        node_2.push_str("Hello!");
        node_1.link("node_2", node_2);

        let vector = node_1.get_str_vector_by_key_or("node_2", vec![]);

        assert_eq!(vector[0], "Hello!".to_string());
    }


    #[test]
    fn test_get_str_vector_by_key_or_empty() {
        let node_1 = Node::new();
        let vector = node_1.get_str_vector_by_key_or("node_2", vec!["empty"]);
        assert_eq!(vector[0], "empty".to_string());
    }
}