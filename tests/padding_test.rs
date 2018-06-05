extern crate yoga;

use yoga::{Align, Direction, Edge, Justify, Node, StyleUnit, Undefined};

#[test]
fn test_padding_no_size() {
	let mut root = Node::new();
	root.set_padding(Edge::Left, StyleUnit::Point(10.0.into()));
	root.set_padding(Edge::Top, StyleUnit::Point(10.0.into()));
	root.set_padding(Edge::Right, StyleUnit::Point(10.0.into()));
	root.set_padding(Edge::Bottom, StyleUnit::Point(10.0.into()));
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(20, root.get_layout_width() as i32);
	assert_eq!(20, root.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(20, root.get_layout_width() as i32);
	assert_eq!(20, root.get_layout_height() as i32);
}

#[test]
fn test_padding_container_match_child() {
	let mut root = Node::new();
	root.set_padding(Edge::Left, StyleUnit::Point(10.0.into()));
	root.set_padding(Edge::Top, StyleUnit::Point(10.0.into()));
	root.set_padding(Edge::Right, StyleUnit::Point(10.0.into()));
	root.set_padding(Edge::Bottom, StyleUnit::Point(10.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(10.0.into()));
	root_child0.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(30, root.get_layout_width() as i32);
	assert_eq!(30, root.get_layout_height() as i32);

	assert_eq!(10, root_child0.get_layout_left() as i32);
	assert_eq!(10, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(30, root.get_layout_width() as i32);
	assert_eq!(30, root.get_layout_height() as i32);

	assert_eq!(10, root_child0.get_layout_left() as i32);
	assert_eq!(10, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);
}

#[test]
fn test_padding_flex_child() {
	let mut root = Node::new();
	root.set_padding(Edge::Left, StyleUnit::Point(10.0.into()));
	root.set_padding(Edge::Top, StyleUnit::Point(10.0.into()));
	root.set_padding(Edge::Right, StyleUnit::Point(10.0.into()));
	root.set_padding(Edge::Bottom, StyleUnit::Point(10.0.into()));
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(10, root_child0.get_layout_left() as i32);
	assert_eq!(10, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(80, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(80, root_child0.get_layout_left() as i32);
	assert_eq!(10, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(80, root_child0.get_layout_height() as i32);
}

#[test]
fn test_padding_stretch_child() {
	let mut root = Node::new();
	root.set_padding(Edge::Left, StyleUnit::Point(10.0.into()));
	root.set_padding(Edge::Top, StyleUnit::Point(10.0.into()));
	root.set_padding(Edge::Right, StyleUnit::Point(10.0.into()));
	root.set_padding(Edge::Bottom, StyleUnit::Point(10.0.into()));
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(10, root_child0.get_layout_left() as i32);
	assert_eq!(10, root_child0.get_layout_top() as i32);
	assert_eq!(80, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(10, root_child0.get_layout_left() as i32);
	assert_eq!(10, root_child0.get_layout_top() as i32);
	assert_eq!(80, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);
}

#[test]
fn test_padding_center_child() {
	let mut root = Node::new();
	root.set_justify_content(Justify::Center);
	root.set_align_items(Align::Center);
	root.set_padding(Edge::Start, StyleUnit::Point(10.0.into()));
	root.set_padding(Edge::End, StyleUnit::Point(20.0.into()));
	root.set_padding(Edge::Bottom, StyleUnit::Point(20.0.into()));
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(10.0.into()));
	root_child0.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(40, root_child0.get_layout_left() as i32);
	assert_eq!(35, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(50, root_child0.get_layout_left() as i32);
	assert_eq!(35, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);
}

#[test]
fn test_child_with_padding_align_end() {
	let mut root = Node::new();
	root.set_justify_content(Justify::FlexEnd);
	root.set_align_items(Align::FlexEnd);
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_padding(Edge::Left, StyleUnit::Point(20.0.into()));
	root_child0.set_padding(Edge::Top, StyleUnit::Point(20.0.into()));
	root_child0.set_padding(Edge::Right, StyleUnit::Point(20.0.into()));
	root_child0.set_padding(Edge::Bottom, StyleUnit::Point(20.0.into()));
	root_child0.set_width(StyleUnit::Point(100.0.into()));
	root_child0.set_height(StyleUnit::Point(100.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(100, root_child0.get_layout_left() as i32);
	assert_eq!(100, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(100, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);
}
