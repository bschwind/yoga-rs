extern crate yoga;

use yoga::{Align, Direction, Edge, FlexDirection, Justify, Node, StyleUnit, Undefined, Wrap};

#[test]
fn test_wrap_column() {
	let mut root = Node::new();
	root.set_flex_wrap(Wrap::Wrap);
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(30.0.into()));
	root_child0.set_height(StyleUnit::Point(30.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(30.0.into()));
	root_child1.set_height(StyleUnit::Point(30.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_width(StyleUnit::Point(30.0.into()));
	root_child2.set_height(StyleUnit::Point(30.0.into()));
	root.insert_child(&mut root_child2, 2);

	let mut root_child3 = Node::new();
	root_child3.set_width(StyleUnit::Point(30.0.into()));
	root_child3.set_height(StyleUnit::Point(30.0.into()));
	root.insert_child(&mut root_child3, 3);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(60, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(30, root_child0.get_layout_width() as i32);
	assert_eq!(30, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(30, root_child1.get_layout_top() as i32);
	assert_eq!(30, root_child1.get_layout_width() as i32);
	assert_eq!(30, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(60, root_child2.get_layout_top() as i32);
	assert_eq!(30, root_child2.get_layout_width() as i32);
	assert_eq!(30, root_child2.get_layout_height() as i32);

	assert_eq!(30, root_child3.get_layout_left() as i32);
	assert_eq!(0, root_child3.get_layout_top() as i32);
	assert_eq!(30, root_child3.get_layout_width() as i32);
	assert_eq!(30, root_child3.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(60, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(30, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(30, root_child0.get_layout_width() as i32);
	assert_eq!(30, root_child0.get_layout_height() as i32);

	assert_eq!(30, root_child1.get_layout_left() as i32);
	assert_eq!(30, root_child1.get_layout_top() as i32);
	assert_eq!(30, root_child1.get_layout_width() as i32);
	assert_eq!(30, root_child1.get_layout_height() as i32);

	assert_eq!(30, root_child2.get_layout_left() as i32);
	assert_eq!(60, root_child2.get_layout_top() as i32);
	assert_eq!(30, root_child2.get_layout_width() as i32);
	assert_eq!(30, root_child2.get_layout_height() as i32);

	assert_eq!(0, root_child3.get_layout_left() as i32);
	assert_eq!(0, root_child3.get_layout_top() as i32);
	assert_eq!(30, root_child3.get_layout_width() as i32);
	assert_eq!(30, root_child3.get_layout_height() as i32);
}

#[test]
fn test_wrap_row() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_flex_wrap(Wrap::Wrap);
	root.set_width(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(30.0.into()));
	root_child0.set_height(StyleUnit::Point(30.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(30.0.into()));
	root_child1.set_height(StyleUnit::Point(30.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_width(StyleUnit::Point(30.0.into()));
	root_child2.set_height(StyleUnit::Point(30.0.into()));
	root.insert_child(&mut root_child2, 2);

	let mut root_child3 = Node::new();
	root_child3.set_width(StyleUnit::Point(30.0.into()));
	root_child3.set_height(StyleUnit::Point(30.0.into()));
	root.insert_child(&mut root_child3, 3);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(60, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(30, root_child0.get_layout_width() as i32);
	assert_eq!(30, root_child0.get_layout_height() as i32);

	assert_eq!(30, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(30, root_child1.get_layout_width() as i32);
	assert_eq!(30, root_child1.get_layout_height() as i32);

	assert_eq!(60, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(30, root_child2.get_layout_width() as i32);
	assert_eq!(30, root_child2.get_layout_height() as i32);

	assert_eq!(0, root_child3.get_layout_left() as i32);
	assert_eq!(30, root_child3.get_layout_top() as i32);
	assert_eq!(30, root_child3.get_layout_width() as i32);
	assert_eq!(30, root_child3.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(60, root.get_layout_height() as i32);

	assert_eq!(70, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(30, root_child0.get_layout_width() as i32);
	assert_eq!(30, root_child0.get_layout_height() as i32);

	assert_eq!(40, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(30, root_child1.get_layout_width() as i32);
	assert_eq!(30, root_child1.get_layout_height() as i32);

	assert_eq!(10, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(30, root_child2.get_layout_width() as i32);
	assert_eq!(30, root_child2.get_layout_height() as i32);

	assert_eq!(70, root_child3.get_layout_left() as i32);
	assert_eq!(30, root_child3.get_layout_top() as i32);
	assert_eq!(30, root_child3.get_layout_width() as i32);
	assert_eq!(30, root_child3.get_layout_height() as i32);
}

#[test]
fn test_wrap_row_align_items_flex_end() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_align_items(Align::FlexEnd);
	root.set_flex_wrap(Wrap::Wrap);
	root.set_width(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(30.0.into()));
	root_child0.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(30.0.into()));
	root_child1.set_height(StyleUnit::Point(20.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_width(StyleUnit::Point(30.0.into()));
	root_child2.set_height(StyleUnit::Point(30.0.into()));
	root.insert_child(&mut root_child2, 2);

	let mut root_child3 = Node::new();
	root_child3.set_width(StyleUnit::Point(30.0.into()));
	root_child3.set_height(StyleUnit::Point(30.0.into()));
	root.insert_child(&mut root_child3, 3);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(60, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(20, root_child0.get_layout_top() as i32);
	assert_eq!(30, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(30, root_child1.get_layout_left() as i32);
	assert_eq!(10, root_child1.get_layout_top() as i32);
	assert_eq!(30, root_child1.get_layout_width() as i32);
	assert_eq!(20, root_child1.get_layout_height() as i32);

	assert_eq!(60, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(30, root_child2.get_layout_width() as i32);
	assert_eq!(30, root_child2.get_layout_height() as i32);

	assert_eq!(0, root_child3.get_layout_left() as i32);
	assert_eq!(30, root_child3.get_layout_top() as i32);
	assert_eq!(30, root_child3.get_layout_width() as i32);
	assert_eq!(30, root_child3.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(60, root.get_layout_height() as i32);

	assert_eq!(70, root_child0.get_layout_left() as i32);
	assert_eq!(20, root_child0.get_layout_top() as i32);
	assert_eq!(30, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(40, root_child1.get_layout_left() as i32);
	assert_eq!(10, root_child1.get_layout_top() as i32);
	assert_eq!(30, root_child1.get_layout_width() as i32);
	assert_eq!(20, root_child1.get_layout_height() as i32);

	assert_eq!(10, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(30, root_child2.get_layout_width() as i32);
	assert_eq!(30, root_child2.get_layout_height() as i32);

	assert_eq!(70, root_child3.get_layout_left() as i32);
	assert_eq!(30, root_child3.get_layout_top() as i32);
	assert_eq!(30, root_child3.get_layout_width() as i32);
	assert_eq!(30, root_child3.get_layout_height() as i32);
}

#[test]
fn test_wrap_row_align_items_center() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_align_items(Align::Center);
	root.set_flex_wrap(Wrap::Wrap);
	root.set_width(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(30.0.into()));
	root_child0.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(30.0.into()));
	root_child1.set_height(StyleUnit::Point(20.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_width(StyleUnit::Point(30.0.into()));
	root_child2.set_height(StyleUnit::Point(30.0.into()));
	root.insert_child(&mut root_child2, 2);

	let mut root_child3 = Node::new();
	root_child3.set_width(StyleUnit::Point(30.0.into()));
	root_child3.set_height(StyleUnit::Point(30.0.into()));
	root.insert_child(&mut root_child3, 3);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(60, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(10, root_child0.get_layout_top() as i32);
	assert_eq!(30, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(30, root_child1.get_layout_left() as i32);
	assert_eq!(5, root_child1.get_layout_top() as i32);
	assert_eq!(30, root_child1.get_layout_width() as i32);
	assert_eq!(20, root_child1.get_layout_height() as i32);

	assert_eq!(60, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(30, root_child2.get_layout_width() as i32);
	assert_eq!(30, root_child2.get_layout_height() as i32);

	assert_eq!(0, root_child3.get_layout_left() as i32);
	assert_eq!(30, root_child3.get_layout_top() as i32);
	assert_eq!(30, root_child3.get_layout_width() as i32);
	assert_eq!(30, root_child3.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(60, root.get_layout_height() as i32);

	assert_eq!(70, root_child0.get_layout_left() as i32);
	assert_eq!(10, root_child0.get_layout_top() as i32);
	assert_eq!(30, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(40, root_child1.get_layout_left() as i32);
	assert_eq!(5, root_child1.get_layout_top() as i32);
	assert_eq!(30, root_child1.get_layout_width() as i32);
	assert_eq!(20, root_child1.get_layout_height() as i32);

	assert_eq!(10, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(30, root_child2.get_layout_width() as i32);
	assert_eq!(30, root_child2.get_layout_height() as i32);

	assert_eq!(70, root_child3.get_layout_left() as i32);
	assert_eq!(30, root_child3.get_layout_top() as i32);
	assert_eq!(30, root_child3.get_layout_width() as i32);
	assert_eq!(30, root_child3.get_layout_height() as i32);
}

#[test]
fn test_flex_wrap_children_with_min_main_overriding_flex_basis() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_flex_wrap(Wrap::Wrap);
	root.set_width(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_basis(StyleUnit::Point(50.0.into()));
	root_child0.set_min_width(StyleUnit::Point(55.0.into()));
	root_child0.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_flex_basis(StyleUnit::Point(50.0.into()));
	root_child1.set_min_width(StyleUnit::Point(55.0.into()));
	root_child1.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(55, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(50, root_child1.get_layout_top() as i32);
	assert_eq!(55, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(45, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(55, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(45, root_child1.get_layout_left() as i32);
	assert_eq!(50, root_child1.get_layout_top() as i32);
	assert_eq!(55, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);
}

#[test]
fn test_flex_wrap_wrap_to_child_height() {
	let mut root = Node::new();

	let mut root_child0 = Node::new();
	root_child0.set_flex_direction(FlexDirection::Row);
	root_child0.set_align_items(Align::FlexStart);
	root_child0.set_flex_wrap(Wrap::Wrap);
	root.insert_child(&mut root_child0, 0);

	let mut root_child0_child0 = Node::new();
	root_child0_child0.set_width(StyleUnit::Point(100.0.into()));
	root_child0.insert_child(&mut root_child0_child0, 0);

	let mut root_child0_child0_child0 = Node::new();
	root_child0_child0_child0.set_width(StyleUnit::Point(100.0.into()));
	root_child0_child0_child0.set_height(StyleUnit::Point(100.0.into()));
	root_child0_child0.insert_child(&mut root_child0_child0_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(100.0.into()));
	root_child1.set_height(StyleUnit::Point(100.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0_child0_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0_child0_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(100, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(100, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0_child0_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0_child0_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(100, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(100, root_child1.get_layout_height() as i32);
}

#[test]
fn test_flex_wrap_align_stretch_fits_one_row() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_flex_wrap(Wrap::Wrap);
	root.set_width(StyleUnit::Point(150.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(150, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	assert_eq!(50, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(100, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(150, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(100, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	assert_eq!(50, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(100, root_child1.get_layout_height() as i32);
}

#[test]
fn test_wrap_reverse_row_align_content_flex_start() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_flex_wrap(Wrap::WrapReverse);
	root.set_width(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(30.0.into()));
	root_child0.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(30.0.into()));
	root_child1.set_height(StyleUnit::Point(20.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_width(StyleUnit::Point(30.0.into()));
	root_child2.set_height(StyleUnit::Point(30.0.into()));
	root.insert_child(&mut root_child2, 2);

	let mut root_child3 = Node::new();
	root_child3.set_width(StyleUnit::Point(30.0.into()));
	root_child3.set_height(StyleUnit::Point(40.0.into()));
	root.insert_child(&mut root_child3, 3);

	let mut root_child4 = Node::new();
	root_child4.set_width(StyleUnit::Point(30.0.into()));
	root_child4.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(80, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(70, root_child0.get_layout_top() as i32);
	assert_eq!(30, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(30, root_child1.get_layout_left() as i32);
	assert_eq!(60, root_child1.get_layout_top() as i32);
	assert_eq!(30, root_child1.get_layout_width() as i32);
	assert_eq!(20, root_child1.get_layout_height() as i32);

	assert_eq!(60, root_child2.get_layout_left() as i32);
	assert_eq!(50, root_child2.get_layout_top() as i32);
	assert_eq!(30, root_child2.get_layout_width() as i32);
	assert_eq!(30, root_child2.get_layout_height() as i32);

	assert_eq!(0, root_child3.get_layout_left() as i32);
	assert_eq!(10, root_child3.get_layout_top() as i32);
	assert_eq!(30, root_child3.get_layout_width() as i32);
	assert_eq!(40, root_child3.get_layout_height() as i32);

	assert_eq!(30, root_child4.get_layout_left() as i32);
	assert_eq!(0, root_child4.get_layout_top() as i32);
	assert_eq!(30, root_child4.get_layout_width() as i32);
	assert_eq!(50, root_child4.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(80, root.get_layout_height() as i32);

	assert_eq!(70, root_child0.get_layout_left() as i32);
	assert_eq!(70, root_child0.get_layout_top() as i32);
	assert_eq!(30, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(40, root_child1.get_layout_left() as i32);
	assert_eq!(60, root_child1.get_layout_top() as i32);
	assert_eq!(30, root_child1.get_layout_width() as i32);
	assert_eq!(20, root_child1.get_layout_height() as i32);

	assert_eq!(10, root_child2.get_layout_left() as i32);
	assert_eq!(50, root_child2.get_layout_top() as i32);
	assert_eq!(30, root_child2.get_layout_width() as i32);
	assert_eq!(30, root_child2.get_layout_height() as i32);

	assert_eq!(70, root_child3.get_layout_left() as i32);
	assert_eq!(10, root_child3.get_layout_top() as i32);
	assert_eq!(30, root_child3.get_layout_width() as i32);
	assert_eq!(40, root_child3.get_layout_height() as i32);

	assert_eq!(40, root_child4.get_layout_left() as i32);
	assert_eq!(0, root_child4.get_layout_top() as i32);
	assert_eq!(30, root_child4.get_layout_width() as i32);
	assert_eq!(50, root_child4.get_layout_height() as i32);
}

#[test]
fn test_wrap_reverse_row_align_content_center() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_align_content(Align::Center);
	root.set_flex_wrap(Wrap::WrapReverse);
	root.set_width(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(30.0.into()));
	root_child0.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(30.0.into()));
	root_child1.set_height(StyleUnit::Point(20.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_width(StyleUnit::Point(30.0.into()));
	root_child2.set_height(StyleUnit::Point(30.0.into()));
	root.insert_child(&mut root_child2, 2);

	let mut root_child3 = Node::new();
	root_child3.set_width(StyleUnit::Point(30.0.into()));
	root_child3.set_height(StyleUnit::Point(40.0.into()));
	root.insert_child(&mut root_child3, 3);

	let mut root_child4 = Node::new();
	root_child4.set_width(StyleUnit::Point(30.0.into()));
	root_child4.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(80, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(70, root_child0.get_layout_top() as i32);
	assert_eq!(30, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(30, root_child1.get_layout_left() as i32);
	assert_eq!(60, root_child1.get_layout_top() as i32);
	assert_eq!(30, root_child1.get_layout_width() as i32);
	assert_eq!(20, root_child1.get_layout_height() as i32);

	assert_eq!(60, root_child2.get_layout_left() as i32);
	assert_eq!(50, root_child2.get_layout_top() as i32);
	assert_eq!(30, root_child2.get_layout_width() as i32);
	assert_eq!(30, root_child2.get_layout_height() as i32);

	assert_eq!(0, root_child3.get_layout_left() as i32);
	assert_eq!(10, root_child3.get_layout_top() as i32);
	assert_eq!(30, root_child3.get_layout_width() as i32);
	assert_eq!(40, root_child3.get_layout_height() as i32);

	assert_eq!(30, root_child4.get_layout_left() as i32);
	assert_eq!(0, root_child4.get_layout_top() as i32);
	assert_eq!(30, root_child4.get_layout_width() as i32);
	assert_eq!(50, root_child4.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(80, root.get_layout_height() as i32);

	assert_eq!(70, root_child0.get_layout_left() as i32);
	assert_eq!(70, root_child0.get_layout_top() as i32);
	assert_eq!(30, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(40, root_child1.get_layout_left() as i32);
	assert_eq!(60, root_child1.get_layout_top() as i32);
	assert_eq!(30, root_child1.get_layout_width() as i32);
	assert_eq!(20, root_child1.get_layout_height() as i32);

	assert_eq!(10, root_child2.get_layout_left() as i32);
	assert_eq!(50, root_child2.get_layout_top() as i32);
	assert_eq!(30, root_child2.get_layout_width() as i32);
	assert_eq!(30, root_child2.get_layout_height() as i32);

	assert_eq!(70, root_child3.get_layout_left() as i32);
	assert_eq!(10, root_child3.get_layout_top() as i32);
	assert_eq!(30, root_child3.get_layout_width() as i32);
	assert_eq!(40, root_child3.get_layout_height() as i32);

	assert_eq!(40, root_child4.get_layout_left() as i32);
	assert_eq!(0, root_child4.get_layout_top() as i32);
	assert_eq!(30, root_child4.get_layout_width() as i32);
	assert_eq!(50, root_child4.get_layout_height() as i32);
}

#[test]
fn test_wrap_reverse_row_single_line_different_size() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_flex_wrap(Wrap::WrapReverse);
	root.set_width(StyleUnit::Point(300.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(30.0.into()));
	root_child0.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(30.0.into()));
	root_child1.set_height(StyleUnit::Point(20.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_width(StyleUnit::Point(30.0.into()));
	root_child2.set_height(StyleUnit::Point(30.0.into()));
	root.insert_child(&mut root_child2, 2);

	let mut root_child3 = Node::new();
	root_child3.set_width(StyleUnit::Point(30.0.into()));
	root_child3.set_height(StyleUnit::Point(40.0.into()));
	root.insert_child(&mut root_child3, 3);

	let mut root_child4 = Node::new();
	root_child4.set_width(StyleUnit::Point(30.0.into()));
	root_child4.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(300, root.get_layout_width() as i32);
	assert_eq!(50, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(40, root_child0.get_layout_top() as i32);
	assert_eq!(30, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(30, root_child1.get_layout_left() as i32);
	assert_eq!(30, root_child1.get_layout_top() as i32);
	assert_eq!(30, root_child1.get_layout_width() as i32);
	assert_eq!(20, root_child1.get_layout_height() as i32);

	assert_eq!(60, root_child2.get_layout_left() as i32);
	assert_eq!(20, root_child2.get_layout_top() as i32);
	assert_eq!(30, root_child2.get_layout_width() as i32);
	assert_eq!(30, root_child2.get_layout_height() as i32);

	assert_eq!(90, root_child3.get_layout_left() as i32);
	assert_eq!(10, root_child3.get_layout_top() as i32);
	assert_eq!(30, root_child3.get_layout_width() as i32);
	assert_eq!(40, root_child3.get_layout_height() as i32);

	assert_eq!(120, root_child4.get_layout_left() as i32);
	assert_eq!(0, root_child4.get_layout_top() as i32);
	assert_eq!(30, root_child4.get_layout_width() as i32);
	assert_eq!(50, root_child4.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(300, root.get_layout_width() as i32);
	assert_eq!(50, root.get_layout_height() as i32);

	assert_eq!(270, root_child0.get_layout_left() as i32);
	assert_eq!(40, root_child0.get_layout_top() as i32);
	assert_eq!(30, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(240, root_child1.get_layout_left() as i32);
	assert_eq!(30, root_child1.get_layout_top() as i32);
	assert_eq!(30, root_child1.get_layout_width() as i32);
	assert_eq!(20, root_child1.get_layout_height() as i32);

	assert_eq!(210, root_child2.get_layout_left() as i32);
	assert_eq!(20, root_child2.get_layout_top() as i32);
	assert_eq!(30, root_child2.get_layout_width() as i32);
	assert_eq!(30, root_child2.get_layout_height() as i32);

	assert_eq!(180, root_child3.get_layout_left() as i32);
	assert_eq!(10, root_child3.get_layout_top() as i32);
	assert_eq!(30, root_child3.get_layout_width() as i32);
	assert_eq!(40, root_child3.get_layout_height() as i32);

	assert_eq!(150, root_child4.get_layout_left() as i32);
	assert_eq!(0, root_child4.get_layout_top() as i32);
	assert_eq!(30, root_child4.get_layout_width() as i32);
	assert_eq!(50, root_child4.get_layout_height() as i32);
}

#[test]
fn test_wrap_reverse_row_align_content_stretch() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_align_content(Align::Stretch);
	root.set_flex_wrap(Wrap::WrapReverse);
	root.set_width(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(30.0.into()));
	root_child0.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(30.0.into()));
	root_child1.set_height(StyleUnit::Point(20.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_width(StyleUnit::Point(30.0.into()));
	root_child2.set_height(StyleUnit::Point(30.0.into()));
	root.insert_child(&mut root_child2, 2);

	let mut root_child3 = Node::new();
	root_child3.set_width(StyleUnit::Point(30.0.into()));
	root_child3.set_height(StyleUnit::Point(40.0.into()));
	root.insert_child(&mut root_child3, 3);

	let mut root_child4 = Node::new();
	root_child4.set_width(StyleUnit::Point(30.0.into()));
	root_child4.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(80, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(70, root_child0.get_layout_top() as i32);
	assert_eq!(30, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(30, root_child1.get_layout_left() as i32);
	assert_eq!(60, root_child1.get_layout_top() as i32);
	assert_eq!(30, root_child1.get_layout_width() as i32);
	assert_eq!(20, root_child1.get_layout_height() as i32);

	assert_eq!(60, root_child2.get_layout_left() as i32);
	assert_eq!(50, root_child2.get_layout_top() as i32);
	assert_eq!(30, root_child2.get_layout_width() as i32);
	assert_eq!(30, root_child2.get_layout_height() as i32);

	assert_eq!(0, root_child3.get_layout_left() as i32);
	assert_eq!(10, root_child3.get_layout_top() as i32);
	assert_eq!(30, root_child3.get_layout_width() as i32);
	assert_eq!(40, root_child3.get_layout_height() as i32);

	assert_eq!(30, root_child4.get_layout_left() as i32);
	assert_eq!(0, root_child4.get_layout_top() as i32);
	assert_eq!(30, root_child4.get_layout_width() as i32);
	assert_eq!(50, root_child4.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(80, root.get_layout_height() as i32);

	assert_eq!(70, root_child0.get_layout_left() as i32);
	assert_eq!(70, root_child0.get_layout_top() as i32);
	assert_eq!(30, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(40, root_child1.get_layout_left() as i32);
	assert_eq!(60, root_child1.get_layout_top() as i32);
	assert_eq!(30, root_child1.get_layout_width() as i32);
	assert_eq!(20, root_child1.get_layout_height() as i32);

	assert_eq!(10, root_child2.get_layout_left() as i32);
	assert_eq!(50, root_child2.get_layout_top() as i32);
	assert_eq!(30, root_child2.get_layout_width() as i32);
	assert_eq!(30, root_child2.get_layout_height() as i32);

	assert_eq!(70, root_child3.get_layout_left() as i32);
	assert_eq!(10, root_child3.get_layout_top() as i32);
	assert_eq!(30, root_child3.get_layout_width() as i32);
	assert_eq!(40, root_child3.get_layout_height() as i32);

	assert_eq!(40, root_child4.get_layout_left() as i32);
	assert_eq!(0, root_child4.get_layout_top() as i32);
	assert_eq!(30, root_child4.get_layout_width() as i32);
	assert_eq!(50, root_child4.get_layout_height() as i32);
}

#[test]
fn test_wrap_reverse_row_align_content_space_around() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_align_content(Align::SpaceAround);
	root.set_flex_wrap(Wrap::WrapReverse);
	root.set_width(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(30.0.into()));
	root_child0.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(30.0.into()));
	root_child1.set_height(StyleUnit::Point(20.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_width(StyleUnit::Point(30.0.into()));
	root_child2.set_height(StyleUnit::Point(30.0.into()));
	root.insert_child(&mut root_child2, 2);

	let mut root_child3 = Node::new();
	root_child3.set_width(StyleUnit::Point(30.0.into()));
	root_child3.set_height(StyleUnit::Point(40.0.into()));
	root.insert_child(&mut root_child3, 3);

	let mut root_child4 = Node::new();
	root_child4.set_width(StyleUnit::Point(30.0.into()));
	root_child4.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(80, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(70, root_child0.get_layout_top() as i32);
	assert_eq!(30, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(30, root_child1.get_layout_left() as i32);
	assert_eq!(60, root_child1.get_layout_top() as i32);
	assert_eq!(30, root_child1.get_layout_width() as i32);
	assert_eq!(20, root_child1.get_layout_height() as i32);

	assert_eq!(60, root_child2.get_layout_left() as i32);
	assert_eq!(50, root_child2.get_layout_top() as i32);
	assert_eq!(30, root_child2.get_layout_width() as i32);
	assert_eq!(30, root_child2.get_layout_height() as i32);

	assert_eq!(0, root_child3.get_layout_left() as i32);
	assert_eq!(10, root_child3.get_layout_top() as i32);
	assert_eq!(30, root_child3.get_layout_width() as i32);
	assert_eq!(40, root_child3.get_layout_height() as i32);

	assert_eq!(30, root_child4.get_layout_left() as i32);
	assert_eq!(0, root_child4.get_layout_top() as i32);
	assert_eq!(30, root_child4.get_layout_width() as i32);
	assert_eq!(50, root_child4.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(80, root.get_layout_height() as i32);

	assert_eq!(70, root_child0.get_layout_left() as i32);
	assert_eq!(70, root_child0.get_layout_top() as i32);
	assert_eq!(30, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(40, root_child1.get_layout_left() as i32);
	assert_eq!(60, root_child1.get_layout_top() as i32);
	assert_eq!(30, root_child1.get_layout_width() as i32);
	assert_eq!(20, root_child1.get_layout_height() as i32);

	assert_eq!(10, root_child2.get_layout_left() as i32);
	assert_eq!(50, root_child2.get_layout_top() as i32);
	assert_eq!(30, root_child2.get_layout_width() as i32);
	assert_eq!(30, root_child2.get_layout_height() as i32);

	assert_eq!(70, root_child3.get_layout_left() as i32);
	assert_eq!(10, root_child3.get_layout_top() as i32);
	assert_eq!(30, root_child3.get_layout_width() as i32);
	assert_eq!(40, root_child3.get_layout_height() as i32);

	assert_eq!(40, root_child4.get_layout_left() as i32);
	assert_eq!(0, root_child4.get_layout_top() as i32);
	assert_eq!(30, root_child4.get_layout_width() as i32);
	assert_eq!(50, root_child4.get_layout_height() as i32);
}

#[test]
fn test_wrap_reverse_column_fixed_size() {
	let mut root = Node::new();
	root.set_align_items(Align::Center);
	root.set_flex_wrap(Wrap::WrapReverse);
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(30.0.into()));
	root_child0.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(30.0.into()));
	root_child1.set_height(StyleUnit::Point(20.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_width(StyleUnit::Point(30.0.into()));
	root_child2.set_height(StyleUnit::Point(30.0.into()));
	root.insert_child(&mut root_child2, 2);

	let mut root_child3 = Node::new();
	root_child3.set_width(StyleUnit::Point(30.0.into()));
	root_child3.set_height(StyleUnit::Point(40.0.into()));
	root.insert_child(&mut root_child3, 3);

	let mut root_child4 = Node::new();
	root_child4.set_width(StyleUnit::Point(30.0.into()));
	root_child4.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(170, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(30, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(170, root_child1.get_layout_left() as i32);
	assert_eq!(10, root_child1.get_layout_top() as i32);
	assert_eq!(30, root_child1.get_layout_width() as i32);
	assert_eq!(20, root_child1.get_layout_height() as i32);

	assert_eq!(170, root_child2.get_layout_left() as i32);
	assert_eq!(30, root_child2.get_layout_top() as i32);
	assert_eq!(30, root_child2.get_layout_width() as i32);
	assert_eq!(30, root_child2.get_layout_height() as i32);

	assert_eq!(170, root_child3.get_layout_left() as i32);
	assert_eq!(60, root_child3.get_layout_top() as i32);
	assert_eq!(30, root_child3.get_layout_width() as i32);
	assert_eq!(40, root_child3.get_layout_height() as i32);

	assert_eq!(140, root_child4.get_layout_left() as i32);
	assert_eq!(0, root_child4.get_layout_top() as i32);
	assert_eq!(30, root_child4.get_layout_width() as i32);
	assert_eq!(50, root_child4.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(30, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(10, root_child1.get_layout_top() as i32);
	assert_eq!(30, root_child1.get_layout_width() as i32);
	assert_eq!(20, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(30, root_child2.get_layout_top() as i32);
	assert_eq!(30, root_child2.get_layout_width() as i32);
	assert_eq!(30, root_child2.get_layout_height() as i32);

	assert_eq!(0, root_child3.get_layout_left() as i32);
	assert_eq!(60, root_child3.get_layout_top() as i32);
	assert_eq!(30, root_child3.get_layout_width() as i32);
	assert_eq!(40, root_child3.get_layout_height() as i32);

	assert_eq!(30, root_child4.get_layout_left() as i32);
	assert_eq!(0, root_child4.get_layout_top() as i32);
	assert_eq!(30, root_child4.get_layout_width() as i32);
	assert_eq!(50, root_child4.get_layout_height() as i32);
}

#[test]
fn test_wrapped_row_within_align_items_center() {
	let mut root = Node::new();
	root.set_align_items(Align::Center);
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_direction(FlexDirection::Row);
	root_child0.set_flex_wrap(Wrap::Wrap);
	root.insert_child(&mut root_child0, 0);

	let mut root_child0_child0 = Node::new();
	root_child0_child0.set_width(StyleUnit::Point(150.0.into()));
	root_child0_child0.set_height(StyleUnit::Point(80.0.into()));
	root_child0.insert_child(&mut root_child0_child0, 0);

	let mut root_child0_child1 = Node::new();
	root_child0_child1.set_width(StyleUnit::Point(80.0.into()));
	root_child0_child1.set_height(StyleUnit::Point(80.0.into()));
	root_child0.insert_child(&mut root_child0_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(200, root_child0.get_layout_width() as i32);
	assert_eq!(160, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(150, root_child0_child0.get_layout_width() as i32);
	assert_eq!(80, root_child0_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child1.get_layout_left() as i32);
	assert_eq!(80, root_child0_child1.get_layout_top() as i32);
	assert_eq!(80, root_child0_child1.get_layout_width() as i32);
	assert_eq!(80, root_child0_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(200, root_child0.get_layout_width() as i32);
	assert_eq!(160, root_child0.get_layout_height() as i32);

	assert_eq!(50, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(150, root_child0_child0.get_layout_width() as i32);
	assert_eq!(80, root_child0_child0.get_layout_height() as i32);

	assert_eq!(120, root_child0_child1.get_layout_left() as i32);
	assert_eq!(80, root_child0_child1.get_layout_top() as i32);
	assert_eq!(80, root_child0_child1.get_layout_width() as i32);
	assert_eq!(80, root_child0_child1.get_layout_height() as i32);
}

#[test]
fn test_wrapped_row_within_align_items_flex_start() {
	let mut root = Node::new();
	root.set_align_items(Align::FlexStart);
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_direction(FlexDirection::Row);
	root_child0.set_flex_wrap(Wrap::Wrap);
	root.insert_child(&mut root_child0, 0);

	let mut root_child0_child0 = Node::new();
	root_child0_child0.set_width(StyleUnit::Point(150.0.into()));
	root_child0_child0.set_height(StyleUnit::Point(80.0.into()));
	root_child0.insert_child(&mut root_child0_child0, 0);

	let mut root_child0_child1 = Node::new();
	root_child0_child1.set_width(StyleUnit::Point(80.0.into()));
	root_child0_child1.set_height(StyleUnit::Point(80.0.into()));
	root_child0.insert_child(&mut root_child0_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(200, root_child0.get_layout_width() as i32);
	assert_eq!(160, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(150, root_child0_child0.get_layout_width() as i32);
	assert_eq!(80, root_child0_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child1.get_layout_left() as i32);
	assert_eq!(80, root_child0_child1.get_layout_top() as i32);
	assert_eq!(80, root_child0_child1.get_layout_width() as i32);
	assert_eq!(80, root_child0_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(200, root_child0.get_layout_width() as i32);
	assert_eq!(160, root_child0.get_layout_height() as i32);

	assert_eq!(50, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(150, root_child0_child0.get_layout_width() as i32);
	assert_eq!(80, root_child0_child0.get_layout_height() as i32);

	assert_eq!(120, root_child0_child1.get_layout_left() as i32);
	assert_eq!(80, root_child0_child1.get_layout_top() as i32);
	assert_eq!(80, root_child0_child1.get_layout_width() as i32);
	assert_eq!(80, root_child0_child1.get_layout_height() as i32);
}

#[test]
fn test_wrapped_row_within_align_items_flex_end() {
	let mut root = Node::new();
	root.set_align_items(Align::FlexEnd);
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_direction(FlexDirection::Row);
	root_child0.set_flex_wrap(Wrap::Wrap);
	root.insert_child(&mut root_child0, 0);

	let mut root_child0_child0 = Node::new();
	root_child0_child0.set_width(StyleUnit::Point(150.0.into()));
	root_child0_child0.set_height(StyleUnit::Point(80.0.into()));
	root_child0.insert_child(&mut root_child0_child0, 0);

	let mut root_child0_child1 = Node::new();
	root_child0_child1.set_width(StyleUnit::Point(80.0.into()));
	root_child0_child1.set_height(StyleUnit::Point(80.0.into()));
	root_child0.insert_child(&mut root_child0_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(200, root_child0.get_layout_width() as i32);
	assert_eq!(160, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(150, root_child0_child0.get_layout_width() as i32);
	assert_eq!(80, root_child0_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child1.get_layout_left() as i32);
	assert_eq!(80, root_child0_child1.get_layout_top() as i32);
	assert_eq!(80, root_child0_child1.get_layout_width() as i32);
	assert_eq!(80, root_child0_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(200, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(200, root_child0.get_layout_width() as i32);
	assert_eq!(160, root_child0.get_layout_height() as i32);

	assert_eq!(50, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(150, root_child0_child0.get_layout_width() as i32);
	assert_eq!(80, root_child0_child0.get_layout_height() as i32);

	assert_eq!(120, root_child0_child1.get_layout_left() as i32);
	assert_eq!(80, root_child0_child1.get_layout_top() as i32);
	assert_eq!(80, root_child0_child1.get_layout_width() as i32);
	assert_eq!(80, root_child0_child1.get_layout_height() as i32);
}

#[test]
fn test_wrapped_column_max_height() {
	let mut root = Node::new();
	root.set_justify_content(Justify::Center);
	root.set_align_content(Align::Center);
	root.set_align_items(Align::Center);
	root.set_flex_wrap(Wrap::Wrap);
	root.set_width(StyleUnit::Point(700.0.into()));
	root.set_height(StyleUnit::Point(500.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(100.0.into()));
	root_child0.set_height(StyleUnit::Point(500.0.into()));
	root_child0.set_max_height(StyleUnit::Point(200.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_margin(Edge::Left, StyleUnit::Point(20.0.into()));
	root_child1.set_margin(Edge::Top, StyleUnit::Point(20.0.into()));
	root_child1.set_margin(Edge::Right, StyleUnit::Point(20.0.into()));
	root_child1.set_margin(Edge::Bottom, StyleUnit::Point(20.0.into()));
	root_child1.set_width(StyleUnit::Point(200.0.into()));
	root_child1.set_height(StyleUnit::Point(200.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_width(StyleUnit::Point(100.0.into()));
	root_child2.set_height(StyleUnit::Point(100.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(700, root.get_layout_width() as i32);
	assert_eq!(500, root.get_layout_height() as i32);

	assert_eq!(250, root_child0.get_layout_left() as i32);
	assert_eq!(30, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(200, root_child0.get_layout_height() as i32);

	assert_eq!(200, root_child1.get_layout_left() as i32);
	assert_eq!(250, root_child1.get_layout_top() as i32);
	assert_eq!(200, root_child1.get_layout_width() as i32);
	assert_eq!(200, root_child1.get_layout_height() as i32);

	assert_eq!(420, root_child2.get_layout_left() as i32);
	assert_eq!(200, root_child2.get_layout_top() as i32);
	assert_eq!(100, root_child2.get_layout_width() as i32);
	assert_eq!(100, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(700, root.get_layout_width() as i32);
	assert_eq!(500, root.get_layout_height() as i32);

	assert_eq!(350, root_child0.get_layout_left() as i32);
	assert_eq!(30, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(200, root_child0.get_layout_height() as i32);

	assert_eq!(300, root_child1.get_layout_left() as i32);
	assert_eq!(250, root_child1.get_layout_top() as i32);
	assert_eq!(200, root_child1.get_layout_width() as i32);
	assert_eq!(200, root_child1.get_layout_height() as i32);

	assert_eq!(180, root_child2.get_layout_left() as i32);
	assert_eq!(200, root_child2.get_layout_top() as i32);
	assert_eq!(100, root_child2.get_layout_width() as i32);
	assert_eq!(100, root_child2.get_layout_height() as i32);
}

#[test]
fn test_wrapped_column_max_height_flex() {
	let mut root = Node::new();
	root.set_justify_content(Justify::Center);
	root.set_align_content(Align::Center);
	root.set_align_items(Align::Center);
	root.set_flex_wrap(Wrap::Wrap);
	root.set_width(StyleUnit::Point(700.0.into()));
	root.set_height(StyleUnit::Point(500.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_flex_shrink(1.0);
	root_child0.set_flex_basis(StyleUnit::Percent(0.0.into()));
	root_child0.set_width(StyleUnit::Point(100.0.into()));
	root_child0.set_height(StyleUnit::Point(500.0.into()));
	root_child0.set_max_height(StyleUnit::Point(200.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_flex_grow(1.0);
	root_child1.set_flex_shrink(1.0);
	root_child1.set_flex_basis(StyleUnit::Percent(0.0.into()));
	root_child1.set_margin(Edge::Left, StyleUnit::Point(20.0.into()));
	root_child1.set_margin(Edge::Top, StyleUnit::Point(20.0.into()));
	root_child1.set_margin(Edge::Right, StyleUnit::Point(20.0.into()));
	root_child1.set_margin(Edge::Bottom, StyleUnit::Point(20.0.into()));
	root_child1.set_width(StyleUnit::Point(200.0.into()));
	root_child1.set_height(StyleUnit::Point(200.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_width(StyleUnit::Point(100.0.into()));
	root_child2.set_height(StyleUnit::Point(100.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(700, root.get_layout_width() as i32);
	assert_eq!(500, root.get_layout_height() as i32);

	assert_eq!(300, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(180, root_child0.get_layout_height() as i32);

	assert_eq!(250, root_child1.get_layout_left() as i32);
	assert_eq!(200, root_child1.get_layout_top() as i32);
	assert_eq!(200, root_child1.get_layout_width() as i32);
	assert_eq!(180, root_child1.get_layout_height() as i32);

	assert_eq!(300, root_child2.get_layout_left() as i32);
	assert_eq!(400, root_child2.get_layout_top() as i32);
	assert_eq!(100, root_child2.get_layout_width() as i32);
	assert_eq!(100, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(700, root.get_layout_width() as i32);
	assert_eq!(500, root.get_layout_height() as i32);

	assert_eq!(300, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(180, root_child0.get_layout_height() as i32);

	assert_eq!(250, root_child1.get_layout_left() as i32);
	assert_eq!(200, root_child1.get_layout_top() as i32);
	assert_eq!(200, root_child1.get_layout_width() as i32);
	assert_eq!(180, root_child1.get_layout_height() as i32);

	assert_eq!(300, root_child2.get_layout_left() as i32);
	assert_eq!(400, root_child2.get_layout_top() as i32);
	assert_eq!(100, root_child2.get_layout_width() as i32);
	assert_eq!(100, root_child2.get_layout_height() as i32);
}

#[test]
fn test_wrap_nodes_with_content_sizing_overflowing_margin() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(500.0.into()));
	root.set_height(StyleUnit::Point(500.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_direction(FlexDirection::Row);
	root_child0.set_flex_wrap(Wrap::Wrap);
	root_child0.set_width(StyleUnit::Point(85.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child0_child0 = Node::new();
	root_child0.insert_child(&mut root_child0_child0, 0);

	let mut root_child0_child0_child0 = Node::new();
	root_child0_child0_child0.set_width(StyleUnit::Point(40.0.into()));
	root_child0_child0_child0.set_height(StyleUnit::Point(40.0.into()));
	root_child0_child0.insert_child(&mut root_child0_child0_child0, 0);

	let mut root_child0_child1 = Node::new();
	root_child0_child1.set_margin(Edge::Right, StyleUnit::Point(10.0.into()));
	root_child0.insert_child(&mut root_child0_child1, 1);

	let mut root_child0_child1_child0 = Node::new();
	root_child0_child1_child0.set_width(StyleUnit::Point(40.0.into()));
	root_child0_child1_child0.set_height(StyleUnit::Point(40.0.into()));
	root_child0_child1.insert_child(&mut root_child0_child1_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(500, root.get_layout_width() as i32);
	assert_eq!(500, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(85, root_child0.get_layout_width() as i32);
	assert_eq!(80, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(40, root_child0_child0.get_layout_width() as i32);
	assert_eq!(40, root_child0_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0_child0.get_layout_top() as i32);
	assert_eq!(40, root_child0_child0_child0.get_layout_width() as i32);
	assert_eq!(40, root_child0_child0_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child1.get_layout_left() as i32);
	assert_eq!(40, root_child0_child1.get_layout_top() as i32);
	assert_eq!(40, root_child0_child1.get_layout_width() as i32);
	assert_eq!(40, root_child0_child1.get_layout_height() as i32);

	assert_eq!(0, root_child0_child1_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child1_child0.get_layout_top() as i32);
	assert_eq!(40, root_child0_child1_child0.get_layout_width() as i32);
	assert_eq!(40, root_child0_child1_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(500, root.get_layout_width() as i32);
	assert_eq!(500, root.get_layout_height() as i32);

	assert_eq!(415, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(85, root_child0.get_layout_width() as i32);
	assert_eq!(80, root_child0.get_layout_height() as i32);

	assert_eq!(45, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(40, root_child0_child0.get_layout_width() as i32);
	assert_eq!(40, root_child0_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0_child0.get_layout_top() as i32);
	assert_eq!(40, root_child0_child0_child0.get_layout_width() as i32);
	assert_eq!(40, root_child0_child0_child0.get_layout_height() as i32);

	assert_eq!(35, root_child0_child1.get_layout_left() as i32);
	assert_eq!(40, root_child0_child1.get_layout_top() as i32);
	assert_eq!(40, root_child0_child1.get_layout_width() as i32);
	assert_eq!(40, root_child0_child1.get_layout_height() as i32);

	assert_eq!(0, root_child0_child1_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child1_child0.get_layout_top() as i32);
	assert_eq!(40, root_child0_child1_child0.get_layout_width() as i32);
	assert_eq!(40, root_child0_child1_child0.get_layout_height() as i32);
}

#[test]
fn test_wrap_nodes_with_content_sizing_margin_cross() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(500.0.into()));
	root.set_height(StyleUnit::Point(500.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_direction(FlexDirection::Row);
	root_child0.set_flex_wrap(Wrap::Wrap);
	root_child0.set_width(StyleUnit::Point(70.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child0_child0 = Node::new();
	root_child0.insert_child(&mut root_child0_child0, 0);

	let mut root_child0_child0_child0 = Node::new();
	root_child0_child0_child0.set_width(StyleUnit::Point(40.0.into()));
	root_child0_child0_child0.set_height(StyleUnit::Point(40.0.into()));
	root_child0_child0.insert_child(&mut root_child0_child0_child0, 0);

	let mut root_child0_child1 = Node::new();
	root_child0_child1.set_margin(Edge::Top, StyleUnit::Point(10.0.into()));
	root_child0.insert_child(&mut root_child0_child1, 1);

	let mut root_child0_child1_child0 = Node::new();
	root_child0_child1_child0.set_width(StyleUnit::Point(40.0.into()));
	root_child0_child1_child0.set_height(StyleUnit::Point(40.0.into()));
	root_child0_child1.insert_child(&mut root_child0_child1_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(500, root.get_layout_width() as i32);
	assert_eq!(500, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(70, root_child0.get_layout_width() as i32);
	assert_eq!(90, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(40, root_child0_child0.get_layout_width() as i32);
	assert_eq!(40, root_child0_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0_child0.get_layout_top() as i32);
	assert_eq!(40, root_child0_child0_child0.get_layout_width() as i32);
	assert_eq!(40, root_child0_child0_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child1.get_layout_left() as i32);
	assert_eq!(50, root_child0_child1.get_layout_top() as i32);
	assert_eq!(40, root_child0_child1.get_layout_width() as i32);
	assert_eq!(40, root_child0_child1.get_layout_height() as i32);

	assert_eq!(0, root_child0_child1_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child1_child0.get_layout_top() as i32);
	assert_eq!(40, root_child0_child1_child0.get_layout_width() as i32);
	assert_eq!(40, root_child0_child1_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(500, root.get_layout_width() as i32);
	assert_eq!(500, root.get_layout_height() as i32);

	assert_eq!(430, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(70, root_child0.get_layout_width() as i32);
	assert_eq!(90, root_child0.get_layout_height() as i32);

	assert_eq!(30, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(40, root_child0_child0.get_layout_width() as i32);
	assert_eq!(40, root_child0_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0_child0.get_layout_top() as i32);
	assert_eq!(40, root_child0_child0_child0.get_layout_width() as i32);
	assert_eq!(40, root_child0_child0_child0.get_layout_height() as i32);

	assert_eq!(30, root_child0_child1.get_layout_left() as i32);
	assert_eq!(50, root_child0_child1.get_layout_top() as i32);
	assert_eq!(40, root_child0_child1.get_layout_width() as i32);
	assert_eq!(40, root_child0_child1.get_layout_height() as i32);

	assert_eq!(0, root_child0_child1_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child1_child0.get_layout_top() as i32);
	assert_eq!(40, root_child0_child1_child0.get_layout_width() as i32);
	assert_eq!(40, root_child0_child1_child0.get_layout_height() as i32);
}
