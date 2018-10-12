extern crate yoga;

use yoga::{Direction, FlexDirection, Node, StyleUnit, Undefined};

#[test]
fn test_copy_style_same() {
	let node0 = Node::new();
	let node1 = Node::new();
	assert!(!node0.is_dirty());

	node0.copy_style(&node1);
	assert!(!node0.is_dirty());
}

#[test]
fn test_copy_style_modified() {
	let node0 = Node::new();
	assert!(!node0.is_dirty());
	assert_eq!(FlexDirection::Column, node0.get_flex_direction());
	assert_eq!(node0.get_style_max_height(), StyleUnit::UndefinedValue);

	let mut node1 = Node::new();
	node1.set_flex_direction(FlexDirection::Row);
	node1.set_max_height(StyleUnit::Point(10.0.into()));

	node0.copy_style(&node1);
	assert!(node0.is_dirty());
	assert_eq!(FlexDirection::Row, node0.get_flex_direction());
	assert_eq!(node0.get_style_max_height(), StyleUnit::Point(10.0.into()));
}

#[test]
fn test_copy_style_modified_same() {
	let mut node0 = Node::new();
	node0.set_flex_direction(FlexDirection::Row);
	node0.set_max_height(StyleUnit::Point(10.0.into()));
	node0.calculate_layout(Undefined, Undefined, Direction::LTR);
	assert!(!node0.is_dirty());

	let mut node1 = Node::new();
	node1.set_flex_direction(FlexDirection::Row);
	node1.set_max_height(StyleUnit::Point(10.0.into()));

	node0.copy_style(&node1);
	assert!(!node0.is_dirty());
}
