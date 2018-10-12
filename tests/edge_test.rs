extern crate ordered_float;
#[macro_use]
extern crate yoga;

use yoga::{Direction, FlexDirection, Node, Undefined};
use yoga::prelude::*;

#[test]
fn test_start_overrides() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		FlexGrow(1.0),
		MarginStart(10 pt),
		MarginLeft(20 pt),
		MarginRight(20 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(10.0, child_layout.left());
	assert_eq!(20.0, child_layout.right());
	assert_eq!(0.0, child_layout.top());
	assert_eq!(70.0, child_layout.width());
	assert_eq!(100.0, child_layout.height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(20.0, child_layout.left());
	assert_eq!(10.0, child_layout.right());
	assert_eq!(0.0, child_layout.top());
	assert_eq!(70.0, child_layout.width());
	assert_eq!(100.0, child_layout.height());
}

#[test]
fn test_end_overrides() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		FlexGrow(1.0),
		MarginEnd(10 pt),
		MarginLeft(20 pt),
		MarginRight(20 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(20.0, child_layout.left());
	assert_eq!(10.0, child_layout.right());
	assert_eq!(0.0, child_layout.top());
	assert_eq!(70.0, child_layout.width());
	assert_eq!(100.0, child_layout.height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(10.0, child_layout.left());
	assert_eq!(20.0, child_layout.right());
	assert_eq!(0.0, child_layout.top());
	assert_eq!(70.0, child_layout.width());
	assert_eq!(100.0, child_layout.height());
}

#[test]
fn test_horizontal_overridden() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		FlexGrow(1.0),
		MarginHorizontal(10 pt),
		MarginLeft(20 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(20.0, child_layout.left());
	assert_eq!(10.0, child_layout.right());
	assert_eq!(0.0, child_layout.top());
	assert_eq!(70.0, child_layout.width());
	assert_eq!(100.0, child_layout.height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(20.0, child_layout.left());
	assert_eq!(10.0, child_layout.right());
	assert_eq!(0.0, child_layout.top());
	assert_eq!(70.0, child_layout.width());
	assert_eq!(100.0, child_layout.height());
}

#[test]
fn test_vertical_overridden() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		FlexGrow(1.0),
		MarginVertical(10 pt),
		MarginTop(20 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_layout.left());
	assert_eq!(0.0, child_layout.right());
	assert_eq!(20.0, child_layout.top());
	assert_eq!(10.0, child_layout.bottom());
	assert_eq!(100.0, child_layout.width());
	assert_eq!(70.0, child_layout.height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_layout.left());
	assert_eq!(0.0, child_layout.right());
	assert_eq!(20.0, child_layout.top());
	assert_eq!(10.0, child_layout.bottom());
	assert_eq!(100.0, child_layout.width());
	assert_eq!(70.0, child_layout.height());
}

#[test]
fn test_horizontal_overrides_all() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		FlexGrow(1.0),
		MarginHorizontal(10 pt),
		Margin(20 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(10.0, child_layout.left());
	assert_eq!(10.0, child_layout.right());
	assert_eq!(20.0, child_layout.top());
	assert_eq!(20.0, child_layout.bottom());
	assert_eq!(80.0, child_layout.width());
	assert_eq!(60.0, child_layout.height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(10.0, child_layout.left());
	assert_eq!(10.0, child_layout.right());
	assert_eq!(20.0, child_layout.top());
	assert_eq!(20.0, child_layout.bottom());
	assert_eq!(80.0, child_layout.width());
	assert_eq!(60.0, child_layout.height());
}

#[test]
fn test_vertical_overrides_all() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		FlexGrow(1.0),
		MarginVertical(10 pt),
		Margin(20 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(20.0, child_layout.left());
	assert_eq!(20.0, child_layout.right());
	assert_eq!(10.0, child_layout.top());
	assert_eq!(10.0, child_layout.bottom());
	assert_eq!(60.0, child_layout.width());
	assert_eq!(80.0, child_layout.height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(20.0, child_layout.left());
	assert_eq!(20.0, child_layout.right());
	assert_eq!(10.0, child_layout.top());
	assert_eq!(10.0, child_layout.bottom());
	assert_eq!(60.0, child_layout.width());
	assert_eq!(80.0, child_layout.height());
}

#[test]
fn test_all_overridden() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		FlexGrow(1.0),
		MarginLeft(10 pt),
		MarginTop(10 pt),
		MarginRight(10 pt),
		MarginBottom(10 pt),
		Margin(20 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(10.0, child_layout.left());
	assert_eq!(10.0, child_layout.right());
	assert_eq!(10.0, child_layout.top());
	assert_eq!(10.0, child_layout.bottom());
	assert_eq!(80.0, child_layout.width());
	assert_eq!(80.0, child_layout.height());

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(10.0, child_layout.left());
	assert_eq!(10.0, child_layout.right());
	assert_eq!(10.0, child_layout.top());
	assert_eq!(10.0, child_layout.bottom());
	assert_eq!(80.0, child_layout.width());
	assert_eq!(80.0, child_layout.height());
}
