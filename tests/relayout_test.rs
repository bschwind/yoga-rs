extern crate yoga;

use yoga::{Direction, Node, StyleUnit, Undefined};

#[test]
fn test_recalculate_resolved_dimension_onchange() {
	let mut root = Node::new();

	let mut root_child0 = Node::new();
	root_child0.set_min_height(StyleUnit::Point(10.0.into()));
	root_child0.set_max_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	root_child0.set_min_height(StyleUnit::UndefinedValue);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);
	assert_eq!(0, root_child0.get_layout_height() as i32);
}

#[test]
fn test_relayout_on_drop() {

	let mut root = Node::new();

	{
		let mut root_child0 = Node::new();
		root_child0.set_min_height(StyleUnit::Point(10.0.into()));
		root_child0.set_max_height(StyleUnit::Point(10.0.into()));
		root.insert_child(&mut root_child0, 0);
		root.calculate_layout(Undefined, Undefined, Direction::LTR);
		assert_eq!(10, root.get_layout_height() as i32);
		// root_child0 dropped here
	}

	root.calculate_layout(Undefined, Undefined, Direction::LTR);
	assert_eq!(0, root.get_layout_height() as i32);   // fails, should pass
}