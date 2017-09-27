extern crate yoga;

use yoga::{Align, Direction, Edge, FlexDirection, Justify, Node, StyleUnit, Undefined};

#[test]
fn test_margin_start() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Start, StyleUnit::Point(10.0.into()));
	root_child0.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(10, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(80, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);
}

#[test]
fn test_margin_top() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Top, StyleUnit::Point(10.0.into()));
	root_child0.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(10, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(10, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);
}

#[test]
fn test_margin_end() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_justify_content(Justify::FlexEnd);
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::End, StyleUnit::Point(10.0.into()));
	root_child0.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(80, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(10, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);
}

#[test]
fn test_margin_bottom() {
	let mut root = Node::new();
	root.set_justify_content(Justify::FlexEnd);
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Bottom, StyleUnit::Point(10.0.into()));
	root_child0.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(80, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(80, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);
}

#[test]
fn test_margin_and_flex_row() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_margin(Edge::Start, StyleUnit::Point(10.0.into()));
	root_child0.set_margin(Edge::End, StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(10, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(80, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(10, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(80, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);
}

#[test]
fn test_margin_and_flex_column() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_margin(Edge::Top, StyleUnit::Point(10.0.into()));
	root_child0.set_margin(Edge::Bottom, StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(10, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(80, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(10, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(80, root_child0.get_layout_height() as i32);
}

#[test]
fn test_margin_and_stretch_row() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_margin(Edge::Top, StyleUnit::Point(10.0.into()));
	root_child0.set_margin(Edge::Bottom, StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(10, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(80, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(10, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(80, root_child0.get_layout_height() as i32);
}

#[test]
fn test_margin_and_stretch_column() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_margin(Edge::Start, StyleUnit::Point(10.0.into()));
	root_child0.set_margin(Edge::End, StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(10, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(80, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(10, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(80, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);
}

#[test]
fn test_margin_with_sibling_row() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_margin(Edge::End, StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_flex_grow(1.0);
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(45, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	assert_eq!(55, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(45, root_child1.get_layout_width() as i32);
	assert_eq!(100, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(55, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(45, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(45, root_child1.get_layout_width() as i32);
	assert_eq!(100, root_child1.get_layout_height() as i32);
}

#[test]
fn test_margin_with_sibling_column() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_margin(Edge::Bottom, StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_flex_grow(1.0);
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(45, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(55, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(45, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(45, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(55, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(45, root_child1.get_layout_height() as i32);
}

#[test]
fn test_margin_auto_bottom() {
	let mut root = Node::new();
	root.set_align_items(Align::Center);
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Bottom, StyleUnit::Auto);
	root_child0.set_width(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(50.0.into()));
	root_child1.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(75, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(75, root_child1.get_layout_left() as i32);
	assert_eq!(150, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(75, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(75, root_child1.get_layout_left() as i32);
	assert_eq!(150, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);
}

#[test]
fn test_margin_auto_top() {
	let mut root = Node::new();
	root.set_align_items(Align::Center);
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Top, StyleUnit::Auto);
	root_child0.set_width(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(50.0.into()));
	root_child1.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(75, root_child0.get_layout_left() as i32);
	assert_eq!(100, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(75, root_child1.get_layout_left() as i32);
	assert_eq!(150, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(75, root_child0.get_layout_left() as i32);
	assert_eq!(100, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(75, root_child1.get_layout_left() as i32);
	assert_eq!(150, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);
}

#[test]
fn test_margin_auto_bottom_and_top() {
	let mut root = Node::new();
	root.set_align_items(Align::Center);
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Top, StyleUnit::Auto);
	root_child0.set_margin(Edge::Bottom, StyleUnit::Auto);
	root_child0.set_width(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(50.0.into()));
	root_child1.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(75, root_child0.get_layout_left() as i32);
	assert_eq!(50, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(75, root_child1.get_layout_left() as i32);
	assert_eq!(150, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(75, root_child0.get_layout_left() as i32);
	assert_eq!(50, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(75, root_child1.get_layout_left() as i32);
	assert_eq!(150, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);
}

#[test]
fn test_margin_auto_bottom_and_top_justify_center() {
	let mut root = Node::new();
	root.set_justify_content(Justify::Center);
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Top, StyleUnit::Auto);
	root_child0.set_margin(Edge::Bottom, StyleUnit::Auto);
	root_child0.set_width(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(50.0.into()));
	root_child1.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(50, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(150, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(150, root_child0.get_layout_left() as i32);
	assert_eq!(50, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(150, root_child1.get_layout_left() as i32);
	assert_eq!(150, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);
}

#[test]
fn test_margin_auto_mutiple_children_column() {
	let mut root = Node::new();
	root.set_align_items(Align::Center);
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Top, StyleUnit::Auto);
	root_child0.set_width(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_margin(Edge::Top, StyleUnit::Auto);
	root_child1.set_width(StyleUnit::Point(50.0.into()));
	root_child1.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_width(StyleUnit::Point(50.0.into()));
	root_child2.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(75, root_child0.get_layout_left() as i32);
	assert_eq!(25, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(75, root_child1.get_layout_left() as i32);
	assert_eq!(100, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	assert_eq!(75, root_child2.get_layout_left() as i32);
	assert_eq!(150, root_child2.get_layout_top() as i32);
	assert_eq!(50, root_child2.get_layout_width() as i32);
	assert_eq!(50, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(75, root_child0.get_layout_left() as i32);
	assert_eq!(25, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(75, root_child1.get_layout_left() as i32);
	assert_eq!(100, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	assert_eq!(75, root_child2.get_layout_left() as i32);
	assert_eq!(150, root_child2.get_layout_top() as i32);
	assert_eq!(50, root_child2.get_layout_width() as i32);
	assert_eq!(50, root_child2.get_layout_height() as i32);
}

#[test]
fn test_margin_auto_mutiple_children_row() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_align_items(Align::Center);
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Right, StyleUnit::Auto);
	root_child0.set_width(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_margin(Edge::Right, StyleUnit::Auto);
	root_child1.set_width(StyleUnit::Point(50.0.into()));
	root_child1.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_width(StyleUnit::Point(50.0.into()));
	root_child2.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(75, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(75, root_child1.get_layout_left() as i32);
	assert_eq!(75, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	assert_eq!(150, root_child2.get_layout_left() as i32);
	assert_eq!(75, root_child2.get_layout_top() as i32);
	assert_eq!(50, root_child2.get_layout_width() as i32);
	assert_eq!(50, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(125, root_child0.get_layout_left() as i32);
	assert_eq!(75, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(50, root_child1.get_layout_left() as i32);
	assert_eq!(75, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(75, root_child2.get_layout_top() as i32);
	assert_eq!(50, root_child2.get_layout_width() as i32);
	assert_eq!(50, root_child2.get_layout_height() as i32);
}

#[test]
fn test_margin_auto_left_and_right_column() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_align_items(Align::Center);
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Left, StyleUnit::Auto);
	root_child0.set_margin(Edge::Right, StyleUnit::Auto);
	root_child0.set_width(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(50.0.into()));
	root_child1.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(50, root_child0.get_layout_left() as i32);
	assert_eq!(75, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(150, root_child1.get_layout_left() as i32);
	assert_eq!(75, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(100, root_child0.get_layout_left() as i32);
	assert_eq!(75, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(75, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);
}

#[test]
fn test_margin_auto_left_and_right() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Left, StyleUnit::Auto);
	root_child0.set_margin(Edge::Right, StyleUnit::Auto);
	root_child0.set_width(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(50.0.into()));
	root_child1.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(75, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(50, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(75, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(150, root_child1.get_layout_left() as i32);
	assert_eq!(50, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);
}

#[test]
fn test_margin_auto_start_and_end_column() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_align_items(Align::Center);
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Start, StyleUnit::Auto);
	root_child0.set_margin(Edge::End, StyleUnit::Auto);
	root_child0.set_width(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(50.0.into()));
	root_child1.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(50, root_child0.get_layout_left() as i32);
	assert_eq!(75, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(150, root_child1.get_layout_left() as i32);
	assert_eq!(75, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(100, root_child0.get_layout_left() as i32);
	assert_eq!(75, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(75, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);
}

#[test]
fn test_margin_auto_start_and_end() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Start, StyleUnit::Auto);
	root_child0.set_margin(Edge::End, StyleUnit::Auto);
	root_child0.set_width(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(50.0.into()));
	root_child1.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(75, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(50, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(75, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(150, root_child1.get_layout_left() as i32);
	assert_eq!(50, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);
}

#[test]
fn test_margin_auto_left_and_right_column_and_center() {
	let mut root = Node::new();
	root.set_align_items(Align::Center);
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Left, StyleUnit::Auto);
	root_child0.set_margin(Edge::Right, StyleUnit::Auto);
	root_child0.set_width(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(50.0.into()));
	root_child1.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(75, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(75, root_child1.get_layout_left() as i32);
	assert_eq!(50, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(75, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(75, root_child1.get_layout_left() as i32);
	assert_eq!(50, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);
}

#[test]
fn test_margin_auto_left() {
	let mut root = Node::new();
	root.set_align_items(Align::Center);
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Left, StyleUnit::Auto);
	root_child0.set_width(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(50.0.into()));
	root_child1.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(150, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(75, root_child1.get_layout_left() as i32);
	assert_eq!(50, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(150, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(75, root_child1.get_layout_left() as i32);
	assert_eq!(50, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);
}

#[test]
fn test_margin_auto_right() {
	let mut root = Node::new();
	root.set_align_items(Align::Center);
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Right, StyleUnit::Auto);
	root_child0.set_width(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(50.0.into()));
	root_child1.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(75, root_child1.get_layout_left() as i32);
	assert_eq!(50, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(75, root_child1.get_layout_left() as i32);
	assert_eq!(50, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);
}

#[test]
fn test_margin_auto_left_and_right_strech() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Left, StyleUnit::Auto);
	root_child0.set_margin(Edge::Right, StyleUnit::Auto);
	root_child0.set_width(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(50.0.into()));
	root_child1.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(50, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(150, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(100, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);
}

#[test]
fn test_margin_auto_top_and_bottom_strech() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Top, StyleUnit::Auto);
	root_child0.set_margin(Edge::Bottom, StyleUnit::Auto);
	root_child0.set_width(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(50.0.into()));
	root_child1.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(50, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(150, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(150, root_child0.get_layout_left() as i32);
	assert_eq!(50, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(150, root_child1.get_layout_left() as i32);
	assert_eq!(150, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);
}

#[test]
fn test_margin_should_not_be_part_of_max_height() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(250.0.into()));
	root.set_height(StyleUnit::Point(250.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Top, StyleUnit::Point(20.0.into()));
	root_child0.set_width(StyleUnit::Point(100.0.into()));
	root_child0.set_height(StyleUnit::Point(100.0.into()));
	root_child0.set_max_height(StyleUnit::Point(100.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(250, root.get_layout_width() as i32);
	assert_eq!(250, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(20, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(250, root.get_layout_width() as i32);
	assert_eq!(250, root.get_layout_height() as i32);

	assert_eq!(150, root_child0.get_layout_left() as i32);
	assert_eq!(20, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);
}

#[test]
fn test_margin_should_not_be_part_of_max_width() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(250.0.into()));
	root.set_height(StyleUnit::Point(250.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Left, StyleUnit::Point(20.0.into()));
	root_child0.set_width(StyleUnit::Point(100.0.into()));
	root_child0.set_max_width(StyleUnit::Point(100.0.into()));
	root_child0.set_height(StyleUnit::Point(100.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(250, root.get_layout_width() as i32);
	assert_eq!(250, root.get_layout_height() as i32);

	assert_eq!(20, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(250, root.get_layout_width() as i32);
	assert_eq!(250, root.get_layout_height() as i32);

	assert_eq!(150, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);
}

#[test]
fn test_margin_auto_left_right_child_bigger_than_parent() {
	let mut root = Node::new();
	root.set_justify_content(Justify::Center);
	root.set_width(StyleUnit::Point(52.0.into()));
	root.set_height(StyleUnit::Point(52.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Left, StyleUnit::Auto);
	root_child0.set_margin(Edge::Right, StyleUnit::Auto);
	root_child0.set_width(StyleUnit::Point(72.0.into()));
	root_child0.set_height(StyleUnit::Point(72.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(52, root.get_layout_width() as i32);
	assert_eq!(52, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(-10, root_child0.get_layout_top() as i32);
	assert_eq!(72, root_child0.get_layout_width() as i32);
	assert_eq!(72, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(52, root.get_layout_width() as i32);
	assert_eq!(52, root.get_layout_height() as i32);

	assert_eq!(-20, root_child0.get_layout_left() as i32);
	assert_eq!(-10, root_child0.get_layout_top() as i32);
	assert_eq!(72, root_child0.get_layout_width() as i32);
	assert_eq!(72, root_child0.get_layout_height() as i32);
}

#[test]
fn test_margin_auto_left_child_bigger_than_parent() {
	let mut root = Node::new();
	root.set_justify_content(Justify::Center);
	root.set_width(StyleUnit::Point(52.0.into()));
	root.set_height(StyleUnit::Point(52.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Left, StyleUnit::Auto);
	root_child0.set_width(StyleUnit::Point(72.0.into()));
	root_child0.set_height(StyleUnit::Point(72.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(52, root.get_layout_width() as i32);
	assert_eq!(52, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(-10, root_child0.get_layout_top() as i32);
	assert_eq!(72, root_child0.get_layout_width() as i32);
	assert_eq!(72, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(52, root.get_layout_width() as i32);
	assert_eq!(52, root.get_layout_height() as i32);

	assert_eq!(-20, root_child0.get_layout_left() as i32);
	assert_eq!(-10, root_child0.get_layout_top() as i32);
	assert_eq!(72, root_child0.get_layout_width() as i32);
	assert_eq!(72, root_child0.get_layout_height() as i32);
}

#[test]
fn test_margin_fix_left_auto_right_child_bigger_than_parent() {
	let mut root = Node::new();
	root.set_justify_content(Justify::Center);
	root.set_width(StyleUnit::Point(52.0.into()));
	root.set_height(StyleUnit::Point(52.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Left, StyleUnit::Point(10.0.into()));
	root_child0.set_margin(Edge::Right, StyleUnit::Auto);
	root_child0.set_width(StyleUnit::Point(72.0.into()));
	root_child0.set_height(StyleUnit::Point(72.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(52, root.get_layout_width() as i32);
	assert_eq!(52, root.get_layout_height() as i32);

	assert_eq!(10, root_child0.get_layout_left() as i32);
	assert_eq!(-10, root_child0.get_layout_top() as i32);
	assert_eq!(72, root_child0.get_layout_width() as i32);
	assert_eq!(72, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(52, root.get_layout_width() as i32);
	assert_eq!(52, root.get_layout_height() as i32);

	assert_eq!(-20, root_child0.get_layout_left() as i32);
	assert_eq!(-10, root_child0.get_layout_top() as i32);
	assert_eq!(72, root_child0.get_layout_width() as i32);
	assert_eq!(72, root_child0.get_layout_height() as i32);
}

#[test]
fn test_margin_auto_left_fix_right_child_bigger_than_parent() {
	let mut root = Node::new();
	root.set_justify_content(Justify::Center);
	root.set_width(StyleUnit::Point(52.0.into()));
	root.set_height(StyleUnit::Point(52.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_margin(Edge::Left, StyleUnit::Auto);
	root_child0.set_margin(Edge::Right, StyleUnit::Point(10.0.into()));
	root_child0.set_width(StyleUnit::Point(72.0.into()));
	root_child0.set_height(StyleUnit::Point(72.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(52, root.get_layout_width() as i32);
	assert_eq!(52, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(-10, root_child0.get_layout_top() as i32);
	assert_eq!(72, root_child0.get_layout_width() as i32);
	assert_eq!(72, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(52, root.get_layout_width() as i32);
	assert_eq!(52, root.get_layout_height() as i32);

	assert_eq!(-30, root_child0.get_layout_left() as i32);
	assert_eq!(-10, root_child0.get_layout_top() as i32);
	assert_eq!(72, root_child0.get_layout_width() as i32);
	assert_eq!(72, root_child0.get_layout_height() as i32);
}
