#[macro_use]
extern crate yoga;

use yoga::{Direction, Display, FlexDirection, Node, Percent, Point, Undefined};
use yoga::FlexStyle::*;

#[test]
fn test_display_none() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		FlexGrow(1.0)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		FlexGrow(1.0),
		Display(Display::None)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(100.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(0.0, child_1_layout.width);
	assert_eq!(0.0, child_1_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(100.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(0.0, child_1_layout.width);
	assert_eq!(0.0, child_1_layout.height);
}

#[test]
fn test_display_none_fixed_size() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		FlexGrow(1.0)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(20 pt),
		Height(20 pt),
		Display(Display::None)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(100.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(0.0, child_1_layout.width);
	assert_eq!(0.0, child_1_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(100.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(0.0, child_1_layout.width);
	assert_eq!(0.0, child_1_layout.height);
}

#[test]
fn test_display_none_with_margin() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		MarginLeft(10 pt),
		MarginTop(10 pt),
		MarginRight(10 pt),
		MarginBottom(10 pt),
		Width(20 pt),
		Height(20 pt),
		Display(Display::None)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		FlexGrow(1.0)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(0.0, child_0_layout.width);
	assert_eq!(0.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(100.0, child_1_layout.width);
	assert_eq!(100.0, child_1_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(0.0, child_0_layout.width);
	assert_eq!(0.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(100.0, child_1_layout.width);
	assert_eq!(100.0, child_1_layout.height);
}

#[test]
fn test_display_none_with_child() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		FlexGrow(1.0),
		FlexShrink(1.0),
		FlexBasis(0 %)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		FlexGrow(1.0),
		FlexShrink(1.0),
		FlexBasis(0 %),
		Display(Display::None)
	);

	let mut root_child_1_child_0 = Node::new();

	style!(root_child_1_child_0,
		FlexGrow(1.0),
		FlexShrink(1.0),
		FlexBasis(0 %),
		Width(20 pt),
		MinWidth(0 pt),
		MinHeight(0 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		FlexGrow(1.0),
		FlexShrink(1.0),
		FlexBasis(0 %)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root_child_1.insert_child(&mut root_child_1_child_0, 0);
	root.insert_child(&mut root_child_2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_1_child_0_layout = root_child_1_child_0.get_layout();
	let child_2_layout = root_child_2.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(0.0, child_1_layout.width);
	assert_eq!(0.0, child_1_layout.height);

	assert_eq!(0.0, child_1_child_0_layout.left);
	assert_eq!(0.0, child_1_child_0_layout.top);
	assert_eq!(0.0, child_1_child_0_layout.width);
	assert_eq!(0.0, child_1_child_0_layout.height);

	assert_eq!(50.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(100.0, child_2_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_1_child_0_layout = root_child_1_child_0.get_layout();
	let child_2_layout = root_child_2.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(50.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(0.0, child_1_layout.width);
	assert_eq!(0.0, child_1_layout.height);

	assert_eq!(0.0, child_1_child_0_layout.left);
	assert_eq!(0.0, child_1_child_0_layout.top);
	assert_eq!(0.0, child_1_child_0_layout.width);
	assert_eq!(0.0, child_1_child_0_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(100.0, child_2_layout.height);
}

#[test]
fn test_display_none_with_position() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		FlexGrow(1.0)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		FlexGrow(1.0),
		Top(10 pt),
		Display(Display::None)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(100.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(0.0, child_1_layout.width);
	assert_eq!(0.0, child_1_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(100.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(0.0, child_1_layout.width);
	assert_eq!(0.0, child_1_layout.height);
}
