#[macro_use]
extern crate yoga;

use yoga::{Direction, Node, Point, Undefined};
use yoga::FlexStyle::*;

#[test]
fn test_nested_overflowing_child() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	root.insert_child(&mut root_child_0, 0);

	let mut root_child_0_child_0 = Node::new();

	style!(root_child_0_child_0,
		Width(200 pt),
		Height(200 pt)
	);

	root_child_0.insert_child(&mut root_child_0_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let root_child_0_layout = root_child_0.get_layout();
	let root_child_0_child_0_layout = root_child_0_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, root_child_0_layout.left);
	assert_eq!(0.0, root_child_0_layout.top);
	assert_eq!(100.0, root_child_0_layout.width);
	assert_eq!(200.0, root_child_0_layout.height);

	assert_eq!(0.0, root_child_0_child_0_layout.left);
	assert_eq!(0.0, root_child_0_child_0_layout.top);
	assert_eq!(200.0, root_child_0_child_0_layout.width);
	assert_eq!(200.0, root_child_0_child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let root_child_0_layout = root_child_0.get_layout();
	let root_child_0_child_0_layout = root_child_0_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, root_child_0_layout.left);
	assert_eq!(0.0, root_child_0_layout.top);
	assert_eq!(100.0, root_child_0_layout.width);
	assert_eq!(200.0, root_child_0_layout.height);

	assert_eq!(-100.0, root_child_0_child_0_layout.left);
	assert_eq!(0.0, root_child_0_child_0_layout.top);
	assert_eq!(200.0, root_child_0_child_0_layout.width);
	assert_eq!(200.0, root_child_0_child_0_layout.height);
}

#[test]
fn test_nested_overflowing_child_in_constraint_parent() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(100 pt),
		Height(100 pt)
	);

	root.insert_child(&mut root_child_0, 0);

	let mut root_child_0_child_0 = Node::new();

	style!(root_child_0_child_0,
		Width(200 pt),
		Height(200 pt)
	);

	root_child_0.insert_child(&mut root_child_0_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let root_child_0_layout = root_child_0.get_layout();
	let root_child_0_child_0_layout = root_child_0_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, root_child_0_layout.left);
	assert_eq!(0.0, root_child_0_layout.top);
	assert_eq!(100.0, root_child_0_layout.width);
	assert_eq!(100.0, root_child_0_layout.height);

	assert_eq!(0.0, root_child_0_child_0_layout.left);
	assert_eq!(0.0, root_child_0_child_0_layout.top);
	assert_eq!(200.0, root_child_0_child_0_layout.width);
	assert_eq!(200.0, root_child_0_child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let root_child_0_layout = root_child_0.get_layout();
	let root_child_0_child_0_layout = root_child_0_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, root_child_0_layout.left);
	assert_eq!(0.0, root_child_0_layout.top);
	assert_eq!(100.0, root_child_0_layout.width);
	assert_eq!(100.0, root_child_0_layout.height);

	assert_eq!(-100.0, root_child_0_child_0_layout.left);
	assert_eq!(0.0, root_child_0_child_0_layout.top);
	assert_eq!(200.0, root_child_0_child_0_layout.width);
	assert_eq!(200.0, root_child_0_child_0_layout.height);
}

#[test]
fn test_parent_wrap_child_size_overflowing_parent() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(100 pt)
	);

	root.insert_child(&mut root_child_0, 0);

	let mut root_child_0_child_0 = Node::new();

	style!(root_child_0_child_0,
		Width(100 pt),
		Height(200 pt)
	);

	root_child_0.insert_child(&mut root_child_0_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let root_child_0_layout = root_child_0.get_layout();
	let root_child_0_child_0_layout = root_child_0_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, root_child_0_layout.left);
	assert_eq!(0.0, root_child_0_layout.top);
	assert_eq!(100.0, root_child_0_layout.width);
	assert_eq!(200.0, root_child_0_layout.height);

	assert_eq!(0.0, root_child_0_child_0_layout.left);
	assert_eq!(0.0, root_child_0_child_0_layout.top);
	assert_eq!(100.0, root_child_0_child_0_layout.width);
	assert_eq!(200.0, root_child_0_child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let root_child_0_layout = root_child_0.get_layout();
	let root_child_0_child_0_layout = root_child_0_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, root_child_0_layout.left);
	assert_eq!(0.0, root_child_0_layout.top);
	assert_eq!(100.0, root_child_0_layout.width);
	assert_eq!(200.0, root_child_0_layout.height);

	assert_eq!(0.0, root_child_0_child_0_layout.left);
	assert_eq!(0.0, root_child_0_child_0_layout.top);
	assert_eq!(100.0, root_child_0_child_0_layout.width);
	assert_eq!(200.0, root_child_0_child_0_layout.height);
}
