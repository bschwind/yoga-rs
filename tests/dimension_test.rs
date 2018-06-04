#[macro_use]
extern crate yoga;

use yoga::{Direction, Node, Undefined};
use yoga::prelude::*;

#[test]
fn test_wrap_child() {
	let mut root = Node::new();

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(100 pt),
		Height(100 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
//	assert_eq!(100.0, root_layout.width());
//	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_layout.left());
	assert_eq!(0.0, child_layout.top());
	assert_eq!(100.0, child_layout.width());
	assert_eq!(100.0, child_layout.height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
//	assert_eq!(100.0, root_layout.width());
//	assert_eq!(100.0, root_layout.height());

//	assert_eq!(0.0, child_layout.left());
	assert_eq!(0.0, child_layout.top());
	assert_eq!(100.0, child_layout.width());
	assert_eq!(100.0, child_layout.height());
}

#[test]
fn test_wrap_grandchild() {
	let mut root = Node::new();

	let mut root_child_0 = Node::new();

	let mut root_child_0_child_0 = Node::new();

	style!(root_child_0_child_0,
		Width(100 pt),
		Height(100 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root_child_0.insert_child(&mut root_child_0_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
//	assert_eq!(100.0, root_layout.width());
//	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
//	assert_eq!(100.0, child_0_layout.width());
//	assert_eq!(100.0, child_0_layout.height());

	assert_eq!(0.0, child_0_child_0_layout.left());
	assert_eq!(0.0, child_0_child_0_layout.top());
//	assert_eq!(100.0, child_0_child_0_layout.width());
//	assert_eq!(100.0, child_0_child_0_layout.height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
//	assert_eq!(100.0, root_layout.width());
//	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
//	assert_eq!(100.0, child_0_layout.width());
//	assert_eq!(100.0, child_0_layout.height());

	assert_eq!(0.0, child_0_child_0_layout.left());
	assert_eq!(0.0, child_0_child_0_layout.top());
//	assert_eq!(100.0, child_0_child_0_layout.width());
//	assert_eq!(100.0, child_0_child_0_layout.height());
}
