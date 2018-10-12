extern crate yoga;

use yoga::{Direction, Node, StyleUnit, Undefined};

#[test]
fn test_reset_layout_when_child_removed() {
	let mut root = Node::new();

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(100.0.into()));
	root_child0.set_height(StyleUnit::Point(100.0.into()));
	root.insert_child(&mut root_child0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	root.remove_child(&mut root_child0);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(Undefined, root_child0.get_layout_width());
	assert_eq!(Undefined, root_child0.get_layout_height());
}
