
#[cfg(test)]
pub mod tests {
    use crate::Node;

    pub fn linked_nodes () -> Node {
        let node_1 = Node::new();
        let node_2 = Node::new();

        node_2.link("node_1", node_1.clone());

        node_2
    }
    
    #[test]
    pub fn test_linked_nodes() {
        let node_2 = linked_nodes();
        let node_1 = node_2.node_by_key("node_1");
        
        assert!(node_1.is_some());
    }

    #[test]
    fn test_doc_example() {
        let node_1 = Node::from(None, None);
        node_1.set("key1", "value1");
        node_1.set("key2", "value2");

        let node_2 = Node::new();
        node_2.link("node_1", node_1);

        let node_3 = Node::new();
        node_3.link("node_2", node_2);

        match node_3.node_by_chain(vec!["node_2", "node_1"]) {
            Some(node_1) => {
                for k in node_1.keys().iter() {
                    println!("{:?}", node_1.get(k).unwrap());
                }
            },
            None => println!("No node found by path: [\"node_2\", \"node_1\"]")
        }
    }
}