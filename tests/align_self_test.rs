#[macro_use]
extern crate yoga;

use yoga::{Align, Direction, FlexDirection, Node, Undefined};
use yoga::prelude::*;

#[test]
fn test_align_self_center() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		AlignSelf(Align::Center),
		Width(10 pt),
		Height(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(45.0, child_layout.left);
	assert_eq!(0.0, child_layout.top);
	assert_eq!(10.0, child_layout.width);
	assert_eq!(10.0, child_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(45.0, child_layout.left);
	assert_eq!(0.0, child_layout.top);
	assert_eq!(10.0, child_layout.width);
	assert_eq!(10.0, child_layout.height);
}

#[test]
fn test_align_self_flex_end() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		AlignSelf(Align::FlexEnd),
		Width(10 pt),
		Height(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(90.0, child_layout.left);
	assert_eq!(0.0, child_layout.top);
	assert_eq!(10.0, child_layout.width);
	assert_eq!(10.0, child_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_layout.left);
	assert_eq!(0.0, child_layout.top);
	assert_eq!(10.0, child_layout.width);
	assert_eq!(10.0, child_layout.height);
}

#[test]
fn test_align_self_flex_start() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		AlignSelf(Align::FlexStart),
		Width(10 pt),
		Height(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_layout.left);
	assert_eq!(0.0, child_layout.top);
	assert_eq!(10.0, child_layout.width);
	assert_eq!(10.0, child_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(90.0, child_layout.left);
	assert_eq!(0.0, child_layout.top);
	assert_eq!(10.0, child_layout.width);
	assert_eq!(10.0, child_layout.height);
}

#[test]
fn test_align_self_flex_end_override_flex_start() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		AlignSelf(Align::FlexEnd),
		Width(10 pt),
		Height(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(90.0, child_layout.left);
	assert_eq!(0.0, child_layout.top);
	assert_eq!(10.0, child_layout.width);
	assert_eq!(10.0, child_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_layout.left);
	assert_eq!(0.0, child_layout.top);
	assert_eq!(10.0, child_layout.width);
	assert_eq!(10.0, child_layout.height);
}

#[test]
fn test_align_self_baseline() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		AlignSelf(Align::Baseline),
		Width(50 pt),
		Height(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		AlignSelf(Align::Baseline),
		Width(50 pt),
		Height(20 pt)
	);

	let mut root_child_1_child_0 = Node::new();

	style!(root_child_1_child_0,
		Width(50 pt),
		Height(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);

	root_child_1.insert_child(&mut root_child_1_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_1_child_0_layout = root_child_1_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(40.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(20.0, child_1_layout.height);

	assert_eq!(0.0, child_1_child_0_layout.left);
	assert_eq!(0.0, child_1_child_0_layout.top);
	assert_eq!(50.0, child_1_child_0_layout.width);
	assert_eq!(10.0, child_1_child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_1_child_0_layout = root_child_1_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(50.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(40.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(20.0, child_1_layout.height);

	assert_eq!(0.0, child_1_child_0_layout.left);
	assert_eq!(0.0, child_1_child_0_layout.top);
	assert_eq!(50.0, child_1_child_0_layout.width);
	assert_eq!(10.0, child_1_child_0_layout.height);
}
