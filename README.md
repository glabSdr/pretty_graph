# Hello pretty_graph 🎀

pretty_graph provides simple toolbox to build and working with graphs.

Struct Node is main operation element - technically link to StrNodeBody (private struct).

StrNodeBody can contains key -> value fields. StrNodeBody supports only fields type &'static str.

StrNodeBody can contains key -> Node fields.
 

## Simple example to start
```rust
    use heap_node::Node;

    fn main() {
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
```