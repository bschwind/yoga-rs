extern crate yoga;

use yoga::Node;

#[test]
fn test_node_drop_order() {
    let mut children = Vec::new();
    let mut root = Node::new();

    for _ in 0..1000 {
        let mut c = Node::new();
        root.insert_child(&mut c, 0);
        children.push(c);
    }
}