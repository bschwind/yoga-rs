#[macro_use]
extern crate yoga;

use yoga::{Direction, FlexDirection, Node, Point, Undefined};
use yoga::FlexStyle::*;

#[test]
fn test_flex_direction_column_no_height() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Height(10 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Height(10 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Height(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(30.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(100.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(10.0, child_1_layout.top);
	assert_eq!(100.0, child_1_layout.width);
	assert_eq!(10.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(20.0, child_2_layout.top);
	assert_eq!(100.0, child_2_layout.width);
	assert_eq!(10.0, child_2_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(30.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(100.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(10.0, child_1_layout.top);
	assert_eq!(100.0, child_1_layout.width);
	assert_eq!(10.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(20.0, child_2_layout.top);
	assert_eq!(100.0, child_2_layout.width);
	assert_eq!(10.0, child_2_layout.height);
}

#[test]
fn test_flex_direction_row_no_width() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(10 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(10 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(30.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(10.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(10.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(10.0, child_1_layout.width);
	assert_eq!(100.0, child_1_layout.height);

	assert_eq!(20.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(10.0, child_2_layout.width);
	assert_eq!(100.0, child_2_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(30.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(20.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(10.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(10.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(10.0, child_1_layout.width);
	assert_eq!(100.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(10.0, child_2_layout.width);
	assert_eq!(100.0, child_2_layout.height);
}

#[test]
fn test_flex_direction_column() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Height(10 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Height(10 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Height(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(100.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(10.0, child_1_layout.top);
	assert_eq!(100.0, child_1_layout.width);
	assert_eq!(10.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(20.0, child_2_layout.top);
	assert_eq!(100.0, child_2_layout.width);
	assert_eq!(10.0, child_2_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(100.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(10.0, child_1_layout.top);
	assert_eq!(100.0, child_1_layout.width);
	assert_eq!(10.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(20.0, child_2_layout.top);
	assert_eq!(100.0, child_2_layout.width);
	assert_eq!(10.0, child_2_layout.height);
}

#[test]
fn test_flex_direction_row() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(10 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(10 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(10.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(10.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(10.0, child_1_layout.width);
	assert_eq!(100.0, child_1_layout.height);

	assert_eq!(20.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(10.0, child_2_layout.width);
	assert_eq!(100.0, child_2_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(90.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(10.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(80.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(10.0, child_1_layout.width);
	assert_eq!(100.0, child_1_layout.height);

	assert_eq!(70.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(10.0, child_2_layout.width);
	assert_eq!(100.0, child_2_layout.height);
}

#[test]
fn test_flex_direction_column_reverse() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::ColumnReverse),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Height(10 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Height(10 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Height(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(90.0, child_0_layout.top);
	assert_eq!(100.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(80.0, child_1_layout.top);
	assert_eq!(100.0, child_1_layout.width);
	assert_eq!(10.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(70.0, child_2_layout.top);
	assert_eq!(100.0, child_2_layout.width);
	assert_eq!(10.0, child_2_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(90.0, child_0_layout.top);
	assert_eq!(100.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(80.0, child_1_layout.top);
	assert_eq!(100.0, child_1_layout.width);
	assert_eq!(10.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(70.0, child_2_layout.top);
	assert_eq!(100.0, child_2_layout.width);
	assert_eq!(10.0, child_2_layout.height);
}

#[test]
fn test_flex_direction_row_reverse() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::RowReverse),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(10 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(10 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(90.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(10.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(80.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(10.0, child_1_layout.width);
	assert_eq!(100.0, child_1_layout.height);

	assert_eq!(70.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(10.0, child_2_layout.width);
	assert_eq!(100.0, child_2_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(10.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(10.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(10.0, child_1_layout.width);
	assert_eq!(100.0, child_1_layout.height);

	assert_eq!(20.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(10.0, child_2_layout.width);
	assert_eq!(100.0, child_2_layout.height);
}
