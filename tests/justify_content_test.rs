extern crate yoga;

use yoga::{Direction, FlexDirection, Justify, Node, StyleUnit, Undefined};

#[test]
fn test_justify_content_row_flex_start() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point(102.0.into()));
	root.set_height(StyleUnit::Point(102.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(102, root.get_layout_width() as i32);
	assert_eq!(102, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(102, root_child0.get_layout_height() as i32);

	assert_eq!(10, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(10, root_child1.get_layout_width() as i32);
	assert_eq!(102, root_child1.get_layout_height() as i32);

	assert_eq!(20, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(10, root_child2.get_layout_width() as i32);
	assert_eq!(102, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(102, root.get_layout_width() as i32);
	assert_eq!(102, root.get_layout_height() as i32);

	assert_eq!(92, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(102, root_child0.get_layout_height() as i32);

	assert_eq!(82, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(10, root_child1.get_layout_width() as i32);
	assert_eq!(102, root_child1.get_layout_height() as i32);

	assert_eq!(72, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(10, root_child2.get_layout_width() as i32);
	assert_eq!(102, root_child2.get_layout_height() as i32);
}

#[test]
fn test_justify_content_row_flex_end() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_justify_content(Justify::FlexEnd);
	root.set_width(StyleUnit::Point(102.0.into()));
	root.set_height(StyleUnit::Point(102.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(102, root.get_layout_width() as i32);
	assert_eq!(102, root.get_layout_height() as i32);

	assert_eq!(72, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(102, root_child0.get_layout_height() as i32);

	assert_eq!(82, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(10, root_child1.get_layout_width() as i32);
	assert_eq!(102, root_child1.get_layout_height() as i32);

	assert_eq!(92, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(10, root_child2.get_layout_width() as i32);
	assert_eq!(102, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(102, root.get_layout_width() as i32);
	assert_eq!(102, root.get_layout_height() as i32);

	assert_eq!(20, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(102, root_child0.get_layout_height() as i32);

	assert_eq!(10, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(10, root_child1.get_layout_width() as i32);
	assert_eq!(102, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(10, root_child2.get_layout_width() as i32);
	assert_eq!(102, root_child2.get_layout_height() as i32);
}

#[test]
fn test_justify_content_row_center() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_justify_content(Justify::Center);
	root.set_width(StyleUnit::Point(102.0.into()));
	root.set_height(StyleUnit::Point(102.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(102, root.get_layout_width() as i32);
	assert_eq!(102, root.get_layout_height() as i32);

	assert_eq!(36, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(102, root_child0.get_layout_height() as i32);

	assert_eq!(46, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(10, root_child1.get_layout_width() as i32);
	assert_eq!(102, root_child1.get_layout_height() as i32);

	assert_eq!(56, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(10, root_child2.get_layout_width() as i32);
	assert_eq!(102, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(102, root.get_layout_width() as i32);
	assert_eq!(102, root.get_layout_height() as i32);

	assert_eq!(56, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(102, root_child0.get_layout_height() as i32);

	assert_eq!(46, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(10, root_child1.get_layout_width() as i32);
	assert_eq!(102, root_child1.get_layout_height() as i32);

	assert_eq!(36, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(10, root_child2.get_layout_width() as i32);
	assert_eq!(102, root_child2.get_layout_height() as i32);
}

#[test]
fn test_justify_content_row_space_between() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_justify_content(Justify::SpaceBetween);
	root.set_width(StyleUnit::Point(102.0.into()));
	root.set_height(StyleUnit::Point(102.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(102, root.get_layout_width() as i32);
	assert_eq!(102, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(102, root_child0.get_layout_height() as i32);

	assert_eq!(46, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(10, root_child1.get_layout_width() as i32);
	assert_eq!(102, root_child1.get_layout_height() as i32);

	assert_eq!(92, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(10, root_child2.get_layout_width() as i32);
	assert_eq!(102, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(102, root.get_layout_width() as i32);
	assert_eq!(102, root.get_layout_height() as i32);

	assert_eq!(92, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(102, root_child0.get_layout_height() as i32);

	assert_eq!(46, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(10, root_child1.get_layout_width() as i32);
	assert_eq!(102, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(10, root_child2.get_layout_width() as i32);
	assert_eq!(102, root_child2.get_layout_height() as i32);
}

#[test]
fn test_justify_content_row_space_around() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_justify_content(Justify::SpaceAround);
	root.set_width(StyleUnit::Point(102.0.into()));
	root.set_height(StyleUnit::Point(102.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(102, root.get_layout_width() as i32);
	assert_eq!(102, root.get_layout_height() as i32);

	assert_eq!(12, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(102, root_child0.get_layout_height() as i32);

	assert_eq!(46, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(10, root_child1.get_layout_width() as i32);
	assert_eq!(102, root_child1.get_layout_height() as i32);

	assert_eq!(80, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(10, root_child2.get_layout_width() as i32);
	assert_eq!(102, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(102, root.get_layout_width() as i32);
	assert_eq!(102, root.get_layout_height() as i32);

	assert_eq!(80, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(102, root_child0.get_layout_height() as i32);

	assert_eq!(46, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(10, root_child1.get_layout_width() as i32);
	assert_eq!(102, root_child1.get_layout_height() as i32);

	assert_eq!(12, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(10, root_child2.get_layout_width() as i32);
	assert_eq!(102, root_child2.get_layout_height() as i32);
}

#[test]
fn test_justify_content_column_flex_start() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(102.0.into()));
	root.set_height(StyleUnit::Point(102.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(102, root.get_layout_width() as i32);
	assert_eq!(102, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(102, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(10, root_child1.get_layout_top() as i32);
	assert_eq!(102, root_child1.get_layout_width() as i32);
	assert_eq!(0, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(10, root_child2.get_layout_top() as i32);
	assert_eq!(102, root_child2.get_layout_width() as i32);
	assert_eq!(10, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(102, root.get_layout_width() as i32);
	assert_eq!(102, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(102, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(10, root_child1.get_layout_top() as i32);
	assert_eq!(102, root_child1.get_layout_width() as i32);
	assert_eq!(0, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(10, root_child2.get_layout_top() as i32);
	assert_eq!(102, root_child2.get_layout_width() as i32);
	assert_eq!(10, root_child2.get_layout_height() as i32);
}

#[test]
fn test_justify_content_column_flex_end() {
	let mut root = Node::new();
	root.set_justify_content(Justify::FlexEnd);
	root.set_width(StyleUnit::Point(102.0.into()));
	root.set_height(StyleUnit::Point(102.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(102, root.get_layout_width() as i32);
	assert_eq!(102, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(72, root_child0.get_layout_top() as i32);
	assert_eq!(102, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(82, root_child1.get_layout_top() as i32);
	assert_eq!(102, root_child1.get_layout_width() as i32);
	assert_eq!(10, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(92, root_child2.get_layout_top() as i32);
	assert_eq!(102, root_child2.get_layout_width() as i32);
	assert_eq!(10, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(102, root.get_layout_width() as i32);
	assert_eq!(102, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(72, root_child0.get_layout_top() as i32);
	assert_eq!(102, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(82, root_child1.get_layout_top() as i32);
	assert_eq!(102, root_child1.get_layout_width() as i32);
	assert_eq!(10, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(92, root_child2.get_layout_top() as i32);
	assert_eq!(102, root_child2.get_layout_width() as i32);
	assert_eq!(10, root_child2.get_layout_height() as i32);
}

#[test]
fn test_justify_content_column_center() {
	let mut root = Node::new();
	root.set_justify_content(Justify::Center);
	root.set_width(StyleUnit::Point(102.0.into()));
	root.set_height(StyleUnit::Point(102.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(102, root.get_layout_width() as i32);
	assert_eq!(102, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(36, root_child0.get_layout_top() as i32);
	assert_eq!(102, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(46, root_child1.get_layout_top() as i32);
	assert_eq!(102, root_child1.get_layout_width() as i32);
	assert_eq!(10, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(56, root_child2.get_layout_top() as i32);
	assert_eq!(102, root_child2.get_layout_width() as i32);
	assert_eq!(10, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(102, root.get_layout_width() as i32);
	assert_eq!(102, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(36, root_child0.get_layout_top() as i32);
	assert_eq!(102, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(46, root_child1.get_layout_top() as i32);
	assert_eq!(102, root_child1.get_layout_width() as i32);
	assert_eq!(10, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(56, root_child2.get_layout_top() as i32);
	assert_eq!(102, root_child2.get_layout_width() as i32);
	assert_eq!(10, root_child2.get_layout_height() as i32);
}

#[test]
fn test_justify_content_column_space_between() {
	let mut root = Node::new();
	root.set_justify_content(Justify::SpaceBetween);
	root.set_width(StyleUnit::Point(102.0.into()));
	root.set_height(StyleUnit::Point(102.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(102, root.get_layout_width() as i32);
	assert_eq!(102, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(102, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(46, root_child1.get_layout_top() as i32);
	assert_eq!(102, root_child1.get_layout_width() as i32);
	assert_eq!(10, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(92, root_child2.get_layout_top() as i32);
	assert_eq!(102, root_child2.get_layout_width() as i32);
	assert_eq!(10, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(102, root.get_layout_width() as i32);
	assert_eq!(102, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(102, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(46, root_child1.get_layout_top() as i32);
	assert_eq!(102, root_child1.get_layout_width() as i32);
	assert_eq!(10, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(92, root_child2.get_layout_top() as i32);
	assert_eq!(102, root_child2.get_layout_width() as i32);
	assert_eq!(10, root_child2.get_layout_height() as i32);
}

#[test]
fn test_justify_content_column_space_around() {
	let mut root = Node::new();
	root.set_justify_content(Justify::SpaceAround);
	root.set_width(StyleUnit::Point(102.0.into()));
	root.set_height(StyleUnit::Point(102.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(102, root.get_layout_width() as i32);
	assert_eq!(102, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(12, root_child0.get_layout_top() as i32);
	assert_eq!(102, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(46, root_child1.get_layout_top() as i32);
	assert_eq!(102, root_child1.get_layout_width() as i32);
	assert_eq!(10, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(80, root_child2.get_layout_top() as i32);
	assert_eq!(102, root_child2.get_layout_width() as i32);
	assert_eq!(10, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(102, root.get_layout_width() as i32);
	assert_eq!(102, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(12, root_child0.get_layout_top() as i32);
	assert_eq!(102, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(46, root_child1.get_layout_top() as i32);
	assert_eq!(102, root_child1.get_layout_width() as i32);
	assert_eq!(10, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(80, root_child2.get_layout_top() as i32);
	assert_eq!(102, root_child2.get_layout_width() as i32);
	assert_eq!(10, root_child2.get_layout_height() as i32);
}
