extern crate yoga;

use yoga::Node;

fn main() {
	let mut node = Node::new();

	let child = Node::new();
	let other_child = Node::new();

	node.insert_child(child, 0);
	node.insert_child(other_child, 1);

	node.set_margin(yoga::Edge::YGEdgeAll, 10.0);
	node.set_padding(yoga::Edge::YGEdgeHorizontal, 4.0);

	node.calculate_layout(512.0, 512.0, yoga::Direction::YGDirectionLTR);

	println!("Layout is {:#?}", node.get_layout());
}
