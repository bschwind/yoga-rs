extern crate yoga;

use yoga::{Align, Direction, FlexDirection, Justify, Node, StyleUnit, Undefined};

#[test]
fn test_max_width() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_max_width(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(50, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);
}

#[test]
fn test_max_height() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(10.0.into()));
	root_child0.set_max_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(90, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);
}

#[test]
fn test_min_height() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_min_height(StyleUnit::Point(60.0.into()));
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
	assert_eq!(80, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(80, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(20, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(80, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(80, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(20, root_child1.get_layout_height() as i32);
}

#[test]
fn test_min_width() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_min_width(StyleUnit::Point(60.0.into()));
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
	assert_eq!(80, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	assert_eq!(80, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(20, root_child1.get_layout_width() as i32);
	assert_eq!(100, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(20, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(80, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(20, root_child1.get_layout_width() as i32);
	assert_eq!(100, root_child1.get_layout_height() as i32);
}

#[test]
fn test_justify_content_min_max() {
	let mut root = Node::new();
	root.set_justify_content(Justify::Center);
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_min_height(StyleUnit::Point(100.0.into()));
	root.set_max_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(60.0.into()));
	root_child0.set_height(StyleUnit::Point(60.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(20, root_child0.get_layout_top() as i32);
	assert_eq!(60, root_child0.get_layout_width() as i32);
	assert_eq!(60, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(40, root_child0.get_layout_left() as i32);
	assert_eq!(20, root_child0.get_layout_top() as i32);
	assert_eq!(60, root_child0.get_layout_width() as i32);
	assert_eq!(60, root_child0.get_layout_height() as i32);
}

#[test]
fn test_align_items_min_max() {
	let mut root = Node::new();
	root.set_align_items(Align::Center);
	root.set_min_width(StyleUnit::Point(100.0.into()));
	root.set_max_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(60.0.into()));
	root_child0.set_height(StyleUnit::Point(60.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(20, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(60, root_child0.get_layout_width() as i32);
	assert_eq!(60, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(20, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(60, root_child0.get_layout_width() as i32);
	assert_eq!(60, root_child0.get_layout_height() as i32);
}

#[test]
fn test_justify_content_overflow_min_max() {
	let mut root = Node::new();
	root.set_justify_content(Justify::Center);
	root.set_min_height(StyleUnit::Point(100.0.into()));
	root.set_max_height(StyleUnit::Point(110.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
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
	assert_eq!(50, root.get_layout_width() as i32);
	assert_eq!(110, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(-20, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(30, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(80, root_child2.get_layout_top() as i32);
	assert_eq!(50, root_child2.get_layout_width() as i32);
	assert_eq!(50, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(50, root.get_layout_width() as i32);
	assert_eq!(110, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(-20, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(30, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(80, root_child2.get_layout_top() as i32);
	assert_eq!(50, root_child2.get_layout_width() as i32);
	assert_eq!(50, root_child2.get_layout_height() as i32);
}

#[test]
fn test_flex_grow_to_min() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_min_height(StyleUnit::Point(100.0.into()));
	root.set_max_height(StyleUnit::Point(500.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_flex_shrink(1.0);
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(50, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(50, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);
}

#[test]
fn test_flex_grow_in_at_most_container() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_align_items(Align::FlexStart);
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_direction(FlexDirection::Row);
	root.insert_child(&mut root_child0, 0);

	let mut root_child0_child0 = Node::new();
	root_child0_child0.set_flex_grow(1.0);
	root_child0_child0.set_flex_basis(StyleUnit::Point(0.0.into()));
	root_child0.insert_child(&mut root_child0_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(0, root_child0.get_layout_width() as i32);
	assert_eq!(0, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(0, root_child0_child0.get_layout_width() as i32);
	assert_eq!(0, root_child0_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(100, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(0, root_child0.get_layout_width() as i32);
	assert_eq!(0, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(0, root_child0_child0.get_layout_width() as i32);
	assert_eq!(0, root_child0_child0.get_layout_height() as i32);
}

#[test]
fn test_flex_grow_child() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_flex_basis(StyleUnit::Point(0.0.into()));
	root_child0.set_height(StyleUnit::Point(100.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(0, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(0, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(0, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(0, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);
}

#[test]
fn test_flex_grow_within_constrained_min_max_column() {
	let mut root = Node::new();
	root.set_min_height(StyleUnit::Point(100.0.into()));
	root.set_max_height(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(0, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(0, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(50, root_child1.get_layout_top() as i32);
	assert_eq!(0, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(0, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(0, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(50, root_child1.get_layout_top() as i32);
	assert_eq!(0, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);
}

#[test]
fn test_flex_grow_within_max_width() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_direction(FlexDirection::Row);
	root_child0.set_max_width(StyleUnit::Point(100.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child0_child0 = Node::new();
	root_child0_child0.set_flex_grow(1.0);
	root_child0_child0.set_height(StyleUnit::Point(20.0.into()));
	root_child0.insert_child(&mut root_child0_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(20, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0_child0.get_layout_width() as i32);
	assert_eq!(20, root_child0_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(100, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(20, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0_child0.get_layout_width() as i32);
	assert_eq!(20, root_child0_child0.get_layout_height() as i32);
}

#[test]
fn test_flex_grow_within_constrained_max_width() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_direction(FlexDirection::Row);
	root_child0.set_max_width(StyleUnit::Point(300.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child0_child0 = Node::new();
	root_child0_child0.set_flex_grow(1.0);
	root_child0_child0.set_height(StyleUnit::Point(20.0.into()));
	root_child0.insert_child(&mut root_child0_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(200, root_child0.get_layout_width() as i32);
	assert_eq!(20, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(200, root_child0_child0.get_layout_width() as i32);
	assert_eq!(20, root_child0_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(200, root_child0.get_layout_width() as i32);
	assert_eq!(20, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(200, root_child0_child0.get_layout_width() as i32);
	assert_eq!(20, root_child0_child0.get_layout_height() as i32);
}

#[test]
fn test_flex_root_ignored() {
	let mut root = Node::new();
	root.set_flex_grow(1.0);
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_min_height(StyleUnit::Point(100.0.into()));
	root.set_max_height(StyleUnit::Point(500.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_flex_basis(StyleUnit::Point(200.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_height(StyleUnit::Point(100.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(300, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(200, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(200, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(100, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(300, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(200, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(200, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(100, root_child1.get_layout_height() as i32);
}

#[test]
fn test_flex_grow_root_minimized() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_min_height(StyleUnit::Point(100.0.into()));
	root.set_max_height(StyleUnit::Point(500.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_min_height(StyleUnit::Point(100.0.into()));
	root_child0.set_max_height(StyleUnit::Point(500.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child0_child0 = Node::new();
	root_child0_child0.set_flex_grow(1.0);
	root_child0_child0.set_flex_basis(StyleUnit::Point(200.0.into()));
	root_child0.insert_child(&mut root_child0_child0, 0);

	let mut root_child0_child1 = Node::new();
	root_child0_child1.set_height(StyleUnit::Point(100.0.into()));
	root_child0.insert_child(&mut root_child0_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(300, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(300, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0_child0.get_layout_width() as i32);
	assert_eq!(200, root_child0_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child1.get_layout_left() as i32);
	assert_eq!(200, root_child0_child1.get_layout_top() as i32);
	assert_eq!(100, root_child0_child1.get_layout_width() as i32);
	assert_eq!(100, root_child0_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(300, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(300, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0_child0.get_layout_width() as i32);
	assert_eq!(200, root_child0_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child1.get_layout_left() as i32);
	assert_eq!(200, root_child0_child1.get_layout_top() as i32);
	assert_eq!(100, root_child0_child1.get_layout_width() as i32);
	assert_eq!(100, root_child0_child1.get_layout_height() as i32);
}

#[test]
fn test_flex_grow_height_maximized() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(500.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_min_height(StyleUnit::Point(100.0.into()));
	root_child0.set_max_height(StyleUnit::Point(500.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child0_child0 = Node::new();
	root_child0_child0.set_flex_grow(1.0);
	root_child0_child0.set_flex_basis(StyleUnit::Point(200.0.into()));
	root_child0.insert_child(&mut root_child0_child0, 0);

	let mut root_child0_child1 = Node::new();
	root_child0_child1.set_height(StyleUnit::Point(100.0.into()));
	root_child0.insert_child(&mut root_child0_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(500, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(500, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0_child0.get_layout_width() as i32);
	assert_eq!(400, root_child0_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child1.get_layout_left() as i32);
	assert_eq!(400, root_child0_child1.get_layout_top() as i32);
	assert_eq!(100, root_child0_child1.get_layout_width() as i32);
	assert_eq!(100, root_child0_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(500, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(500, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0_child0.get_layout_width() as i32);
	assert_eq!(400, root_child0_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child1.get_layout_left() as i32);
	assert_eq!(400, root_child0_child1.get_layout_top() as i32);
	assert_eq!(100, root_child0_child1.get_layout_width() as i32);
	assert_eq!(100, root_child0_child1.get_layout_height() as i32);
}

#[test]
fn test_flex_grow_within_constrained_min_row() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_min_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
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
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(50, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(50, root_child1.get_layout_width() as i32);
	assert_eq!(100, root_child1.get_layout_height() as i32);
}

#[test]
fn test_flex_grow_within_constrained_min_column() {
	let mut root = Node::new();
	root.set_min_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(0, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(0, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(50, root_child1.get_layout_top() as i32);
	assert_eq!(0, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(0, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(0, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(50, root_child1.get_layout_top() as i32);
	assert_eq!(0, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);
}

#[test]
fn test_flex_grow_within_constrained_max_row() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(200.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_direction(FlexDirection::Row);
	root_child0.set_max_width(StyleUnit::Point(100.0.into()));
	root_child0.set_height(StyleUnit::Point(100.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child0_child0 = Node::new();
	root_child0_child0.set_flex_shrink(1.0);
	root_child0_child0.set_flex_basis(StyleUnit::Point(100.0.into()));
	root_child0.insert_child(&mut root_child0_child0, 0);

	let mut root_child0_child1 = Node::new();
	root_child0_child1.set_width(StyleUnit::Point(50.0.into()));
	root_child0.insert_child(&mut root_child0_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0_child0.get_layout_height() as i32);

	assert_eq!(50, root_child0_child1.get_layout_left() as i32);
	assert_eq!(0, root_child0_child1.get_layout_top() as i32);
	assert_eq!(50, root_child0_child1.get_layout_width() as i32);
	assert_eq!(100, root_child0_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(200, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(100, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	assert_eq!(50, root_child0_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0_child0.get_layout_top() as i32);
	assert_eq!(50, root_child0_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child1.get_layout_left() as i32);
	assert_eq!(0, root_child0_child1.get_layout_top() as i32);
	assert_eq!(50, root_child0_child1.get_layout_width() as i32);
	assert_eq!(100, root_child0_child1.get_layout_height() as i32);
}

#[test]
fn test_flex_grow_within_constrained_max_column() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_max_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_shrink(1.0);
	root_child0.set_flex_basis(StyleUnit::Point(100.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_height(StyleUnit::Point(50.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(50, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(50, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);
}

#[test]
fn test_child_min_max_width_flexing() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point(120.0.into()));
	root.set_height(StyleUnit::Point(50.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_flex_basis(StyleUnit::Point(0.0.into()));
	root_child0.set_min_width(StyleUnit::Point(60.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_flex_grow(1.0);
	root_child1.set_flex_basis(StyleUnit::Percent(50.0.into()));
	root_child1.set_max_width(StyleUnit::Point(20.0.into()));
	root.insert_child(&mut root_child1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(120, root.get_layout_width() as i32);
	assert_eq!(50, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(100, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(20, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(120, root.get_layout_width() as i32);
	assert_eq!(50, root.get_layout_height() as i32);

	assert_eq!(20, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(50, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(20, root_child1.get_layout_width() as i32);
	assert_eq!(50, root_child1.get_layout_height() as i32);
}

#[test]
fn test_min_width_overrides_width() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(50.0.into()));
	root.set_min_width(StyleUnit::Point(100.0.into()));
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(0, root.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(0, root.get_layout_height() as i32);
}

#[test]
fn test_max_width_overrides_width() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(200.0.into()));
	root.set_max_width(StyleUnit::Point(100.0.into()));
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(0, root.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(0, root.get_layout_height() as i32);
}

#[test]
fn test_min_height_overrides_height() {
	let mut root = Node::new();
	root.set_height(StyleUnit::Point(50.0.into()));
	root.set_min_height(StyleUnit::Point(100.0.into()));
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(0, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(0, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);
}

#[test]
fn test_max_height_overrides_height() {
	let mut root = Node::new();
	root.set_height(StyleUnit::Point(200.0.into()));
	root.set_max_height(StyleUnit::Point(100.0.into()));
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(0, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(0, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);
}

#[test]
fn test_min_max_percent_no_width_height() {
	let mut root = Node::new();
	root.set_align_items(Align::FlexStart);
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_min_width(StyleUnit::Percent(10.0.into()));
	root_child0.set_max_width(StyleUnit::Percent(10.0.into()));
	root_child0.set_min_height(StyleUnit::Percent(10.0.into()));
	root_child0.set_max_height(StyleUnit::Percent(10.0.into()));
	root.insert_child(&mut root_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(90, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);
}
