#[macro_use]
extern crate yoga;
extern crate ordered_float;

use ordered_float::OrderedFloat;
use yoga::{Align, Direction, FlexDirection, Justify, Node, Point, Undefined, Wrap};
use yoga::FlexStyle::*;

#[test]
fn test_align_items_stretch() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
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
	assert_eq!(100.0, child_layout.width);
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
	assert_eq!(100.0, child_layout.width);
	assert_eq!(10.0, child_layout.height);
}

#[test]
fn test_align_items_center() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::Center),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
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
fn test_align_items_flex_start() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
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
fn test_align_items_flex_end() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexEnd),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
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
fn test_align_baseline() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignItems(Align::Baseline),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(50 pt),
		Height(20 pt)
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
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(30.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(20.0, child_1_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(50.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(30.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(20.0, child_1_layout.height);
}

#[test]
fn test_align_baseline_child() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignItems(Align::Baseline),
		FlexWrap(Wrap::Wrap),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
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

#[test]
fn test_align_baseline_child_multiline() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignItems(Align::Baseline),
		FlexWrap(Wrap::Wrap),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(60 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		FlexDirection(FlexDirection::Row),
		FlexWrap(Wrap::Wrap),
		Width(50 pt),
		Height(25 pt)
	);

	let mut root_child_1_child_0 = Node::new();

	style!(root_child_1_child_0,
		Width(25 pt),
		Height(20 pt)
	);

	let mut root_child_1_child_1 = Node::new();

	style!(root_child_1_child_1,
		Width(25 pt),
		Height(10 pt)
	);

	let mut root_child_1_child_2 = Node::new();

	style!(root_child_1_child_2,
		Width(25 pt),
		Height(20 pt)
	);

	let mut root_child_1_child_3 = Node::new();

	style!(root_child_1_child_3,
		Width(25 pt),
		Height(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);

	root_child_1.insert_child(&mut root_child_1_child_0, 0);
	root_child_1.insert_child(&mut root_child_1_child_1, 1);
	root_child_1.insert_child(&mut root_child_1_child_2, 2);
	root_child_1.insert_child(&mut root_child_1_child_3, 3);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_1_child_0_layout = root_child_1_child_0.get_layout();
	let child_1_child_1_layout = root_child_1_child_1.get_layout();
	let child_1_child_2_layout = root_child_1_child_2.get_layout();
	let child_1_child_3_layout = root_child_1_child_3.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(60.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(40.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(25.0, child_1_layout.height);

	assert_eq!(0.0, child_1_child_0_layout.left);
	assert_eq!(0.0, child_1_child_0_layout.top);
	assert_eq!(25.0, child_1_child_0_layout.width);
	assert_eq!(20.0, child_1_child_0_layout.height);

	assert_eq!(25.0, child_1_child_1_layout.left);
	assert_eq!(0.0, child_1_child_1_layout.top);
	assert_eq!(25.0, child_1_child_1_layout.width);
	assert_eq!(10.0, child_1_child_1_layout.height);

	assert_eq!(0.0, child_1_child_2_layout.left);
	assert_eq!(20.0, child_1_child_2_layout.top);
	assert_eq!(25.0, child_1_child_2_layout.width);
	assert_eq!(20.0, child_1_child_2_layout.height);

	assert_eq!(25.0, child_1_child_3_layout.left);
	assert_eq!(20.0, child_1_child_3_layout.top);
	assert_eq!(25.0, child_1_child_3_layout.width);
	assert_eq!(10.0, child_1_child_3_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_1_child_0_layout = root_child_1_child_0.get_layout();
	let child_1_child_1_layout = root_child_1_child_1.get_layout();
	let child_1_child_2_layout = root_child_1_child_2.get_layout();
	let child_1_child_3_layout = root_child_1_child_3.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(50.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(60.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(40.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(25.0, child_1_layout.height);

	assert_eq!(25.0, child_1_child_0_layout.left);
	assert_eq!(0.0, child_1_child_0_layout.top);
	assert_eq!(25.0, child_1_child_0_layout.width);
	assert_eq!(20.0, child_1_child_0_layout.height);

	assert_eq!(0.0, child_1_child_1_layout.left);
	assert_eq!(0.0, child_1_child_1_layout.top);
	assert_eq!(25.0, child_1_child_1_layout.width);
	assert_eq!(10.0, child_1_child_1_layout.height);

	assert_eq!(25.0, child_1_child_2_layout.left);
	assert_eq!(20.0, child_1_child_2_layout.top);
	assert_eq!(25.0, child_1_child_2_layout.width);
	assert_eq!(20.0, child_1_child_2_layout.height);

	assert_eq!(0.0, child_1_child_3_layout.left);
	assert_eq!(20.0, child_1_child_3_layout.top);
	assert_eq!(25.0, child_1_child_3_layout.width);
	assert_eq!(10.0, child_1_child_3_layout.height);
}

#[test]
fn test_align_baseline_child_multiline_override() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignItems(Align::Baseline),
		FlexWrap(Wrap::Wrap),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(60 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		FlexDirection(FlexDirection::Row),
		FlexWrap(Wrap::Wrap),
		Width(50 pt),
		Height(25 pt)
	);

	let mut root_child_1_child_0 = Node::new();

	style!(root_child_1_child_0,
		Width(25 pt),
		Height(20 pt)
	);

	let mut root_child_1_child_1 = Node::new();

	style!(root_child_1_child_1,
		AlignSelf(Align::Baseline),
		Width(25 pt),
		Height(10 pt)
	);

	let mut root_child_1_child_2 = Node::new();

	style!(root_child_1_child_2,
		Width(25 pt),
		Height(20 pt)
	);

	let mut root_child_1_child_3 = Node::new();

	style!(root_child_1_child_3,
		AlignSelf(Align::Baseline),
		Width(25 pt),
		Height(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);

	root_child_1.insert_child(&mut root_child_1_child_0, 0);
	root_child_1.insert_child(&mut root_child_1_child_1, 1);
	root_child_1.insert_child(&mut root_child_1_child_2, 2);
	root_child_1.insert_child(&mut root_child_1_child_3, 3);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_1_child_0_layout = root_child_1_child_0.get_layout();
	let child_1_child_1_layout = root_child_1_child_1.get_layout();
	let child_1_child_2_layout = root_child_1_child_2.get_layout();
	let child_1_child_3_layout = root_child_1_child_3.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(60.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(50.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(25.0, child_1_layout.height);

	assert_eq!(0.0, child_1_child_0_layout.left);
	assert_eq!(0.0, child_1_child_0_layout.top);
	assert_eq!(25.0, child_1_child_0_layout.width);
	assert_eq!(20.0, child_1_child_0_layout.height);

	assert_eq!(25.0, child_1_child_1_layout.left);
	assert_eq!(0.0, child_1_child_1_layout.top);
	assert_eq!(25.0, child_1_child_1_layout.width);
	assert_eq!(10.0, child_1_child_1_layout.height);

	assert_eq!(0.0, child_1_child_2_layout.left);
	assert_eq!(20.0, child_1_child_2_layout.top);
	assert_eq!(25.0, child_1_child_2_layout.width);
	assert_eq!(20.0, child_1_child_2_layout.height);

	assert_eq!(25.0, child_1_child_3_layout.left);
	assert_eq!(20.0, child_1_child_3_layout.top);
	assert_eq!(25.0, child_1_child_3_layout.width);
	assert_eq!(10.0, child_1_child_3_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_1_child_0_layout = root_child_1_child_0.get_layout();
	let child_1_child_1_layout = root_child_1_child_1.get_layout();
	let child_1_child_2_layout = root_child_1_child_2.get_layout();
	let child_1_child_3_layout = root_child_1_child_3.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(50.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(60.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(50.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(25.0, child_1_layout.height);

	assert_eq!(25.0, child_1_child_0_layout.left);
	assert_eq!(0.0, child_1_child_0_layout.top);
	assert_eq!(25.0, child_1_child_0_layout.width);
	assert_eq!(20.0, child_1_child_0_layout.height);

	assert_eq!(0.0, child_1_child_1_layout.left);
	assert_eq!(0.0, child_1_child_1_layout.top);
	assert_eq!(25.0, child_1_child_1_layout.width);
	assert_eq!(10.0, child_1_child_1_layout.height);

	assert_eq!(25.0, child_1_child_2_layout.left);
	assert_eq!(20.0, child_1_child_2_layout.top);
	assert_eq!(25.0, child_1_child_2_layout.width);
	assert_eq!(20.0, child_1_child_2_layout.height);

	assert_eq!(0.0, child_1_child_3_layout.left);
	assert_eq!(20.0, child_1_child_3_layout.top);
	assert_eq!(25.0, child_1_child_3_layout.width);
	assert_eq!(10.0, child_1_child_3_layout.height);
}

#[test]
fn test_align_baseline_child_multiline_no_override_on_secondline() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignItems(Align::Baseline),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(60 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		FlexDirection(FlexDirection::Row),
		FlexWrap(Wrap::Wrap),
		Width(50 pt),
		Height(25 pt)
	);

	let mut root_child_1_child_0 = Node::new();

	style!(root_child_1_child_0,
		Width(25 pt),
		Height(20 pt)
	);

	let mut root_child_1_child_1 = Node::new();

	style!(root_child_1_child_1,
		Width(25 pt),
		Height(10 pt)
	);

	let mut root_child_1_child_2 = Node::new();

	style!(root_child_1_child_2,
		Width(25 pt),
		Height(20 pt)
	);

	let mut root_child_1_child_3 = Node::new();

	style!(root_child_1_child_3,
		AlignSelf(Align::Baseline),
		Width(25 pt),
		Height(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);

	root_child_1.insert_child(&mut root_child_1_child_0, 0);
	root_child_1.insert_child(&mut root_child_1_child_1, 1);
	root_child_1.insert_child(&mut root_child_1_child_2, 2);
	root_child_1.insert_child(&mut root_child_1_child_3, 3);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_1_child_0_layout = root_child_1_child_0.get_layout();
	let child_1_child_1_layout = root_child_1_child_1.get_layout();
	let child_1_child_2_layout = root_child_1_child_2.get_layout();
	let child_1_child_3_layout = root_child_1_child_3.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(60.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(40.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(25.0, child_1_layout.height);

	assert_eq!(0.0, child_1_child_0_layout.left);
	assert_eq!(0.0, child_1_child_0_layout.top);
	assert_eq!(25.0, child_1_child_0_layout.width);
	assert_eq!(20.0, child_1_child_0_layout.height);

	assert_eq!(25.0, child_1_child_1_layout.left);
	assert_eq!(0.0, child_1_child_1_layout.top);
	assert_eq!(25.0, child_1_child_1_layout.width);
	assert_eq!(10.0, child_1_child_1_layout.height);

	assert_eq!(0.0, child_1_child_2_layout.left);
	assert_eq!(20.0, child_1_child_2_layout.top);
	assert_eq!(25.0, child_1_child_2_layout.width);
	assert_eq!(20.0, child_1_child_2_layout.height);

	assert_eq!(25.0, child_1_child_3_layout.left);
	assert_eq!(20.0, child_1_child_3_layout.top);
	assert_eq!(25.0, child_1_child_3_layout.width);
	assert_eq!(10.0, child_1_child_3_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_1_child_0_layout = root_child_1_child_0.get_layout();
	let child_1_child_1_layout = root_child_1_child_1.get_layout();
	let child_1_child_2_layout = root_child_1_child_2.get_layout();
	let child_1_child_3_layout = root_child_1_child_3.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(50.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(60.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(40.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(25.0, child_1_layout.height);

	assert_eq!(25.0, child_1_child_0_layout.left);
	assert_eq!(0.0, child_1_child_0_layout.top);
	assert_eq!(25.0, child_1_child_0_layout.width);
	assert_eq!(20.0, child_1_child_0_layout.height);

	assert_eq!(0.0, child_1_child_1_layout.left);
	assert_eq!(0.0, child_1_child_1_layout.top);
	assert_eq!(25.0, child_1_child_1_layout.width);
	assert_eq!(10.0, child_1_child_1_layout.height);

	assert_eq!(25.0, child_1_child_2_layout.left);
	assert_eq!(20.0, child_1_child_2_layout.top);
	assert_eq!(25.0, child_1_child_2_layout.width);
	assert_eq!(20.0, child_1_child_2_layout.height);

	assert_eq!(0.0, child_1_child_3_layout.left);
	assert_eq!(20.0, child_1_child_3_layout.top);
	assert_eq!(25.0, child_1_child_3_layout.width);
	assert_eq!(10.0, child_1_child_3_layout.height);
}

#[test]
fn test_align_baseline_child_top() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignItems(Align::Baseline),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Top(10 pt),
		Width(50 pt),
		Height(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
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
	assert_eq!(10.0, child_0_layout.top);
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
	assert_eq!(10.0, child_0_layout.top);
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

#[test]
fn test_align_baseline_child_top2() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignItems(Align::Baseline),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Top(5 pt),
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
	assert_eq!(45.0, child_1_layout.top);
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
	assert_eq!(45.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(20.0, child_1_layout.height);

	assert_eq!(0.0, child_1_child_0_layout.left);
	assert_eq!(0.0, child_1_child_0_layout.top);
	assert_eq!(50.0, child_1_child_0_layout.width);
	assert_eq!(10.0, child_1_child_0_layout.height);
}

#[test]
fn test_align_baseline_double_nested_child() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignItems(Align::Baseline),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(50 pt)
	);

	let mut root_child_0_child_0 = Node::new();

	style!(root_child_0_child_0,
		Width(50 pt),
		Height(20 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(50 pt),
		Height(20 pt)
	);

	let mut root_child_1_child_0 = Node::new();

	style!(root_child_1_child_0,
		Width(50 pt),
		Height(15 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);

	root_child_0.insert_child(&mut root_child_0_child_0, 0);
	root_child_1.insert_child(&mut root_child_1_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0_child_0.get_layout();
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

	assert_eq!(0.0, child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_layout.top);
	assert_eq!(50.0, child_0_child_0_layout.width);
	assert_eq!(20.0, child_0_child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(5.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(20.0, child_1_layout.height);

	assert_eq!(0.0, child_1_child_0_layout.left);
	assert_eq!(0.0, child_1_child_0_layout.top);
	assert_eq!(50.0, child_1_child_0_layout.width);
	assert_eq!(15.0, child_1_child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0_child_0.get_layout();
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

	assert_eq!(0.0, child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_layout.top);
	assert_eq!(50.0, child_0_child_0_layout.width);
	assert_eq!(20.0, child_0_child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(5.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(20.0, child_1_layout.height);

	assert_eq!(0.0, child_1_child_0_layout.left);
	assert_eq!(0.0, child_1_child_0_layout.top);
	assert_eq!(50.0, child_1_child_0_layout.width);
	assert_eq!(15.0, child_1_child_0_layout.height);
}

#[test]
fn test_align_baseline_column() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::Baseline),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(50 pt),
		Height(20 pt)
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
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(50.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(20.0, child_1_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(50.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(50.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(20.0, child_1_layout.height);
}

#[test]
fn test_align_baseline_child_margin() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignItems(Align::Baseline),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		MarginLeft(5 pt),
		MarginTop(5 pt),
		MarginRight(5 pt),
		MarginBottom(5 pt),
		Width(50 pt),
		Height(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(50 pt),
		Height(20 pt)
	);

	let mut root_child_1_child_0 = Node::new();

	style!(root_child_1_child_0,
		MarginLeft(1 pt),
		MarginTop(1 pt),
		MarginRight(1 pt),
		MarginBottom(1 pt),
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

	assert_eq!(5.0, child_0_layout.left);
	assert_eq!(5.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(60.0, child_1_layout.left);
	assert_eq!(44.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(20.0, child_1_layout.height);

	assert_eq!(1.0, child_1_child_0_layout.left);
	assert_eq!(1.0, child_1_child_0_layout.top);
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

	assert_eq!(45.0, child_0_layout.left);
	assert_eq!(5.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(-10.0, child_1_layout.left);
	assert_eq!(44.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(20.0, child_1_layout.height);

	assert_eq!(-1.0, child_1_child_0_layout.left);
	assert_eq!(1.0, child_1_child_0_layout.top);
	assert_eq!(50.0, child_1_child_0_layout.width);
	assert_eq!(10.0, child_1_child_0_layout.height);
}

#[test]
fn test_align_baseline_child_padding() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignItems(Align::Baseline),
		PaddingLeft(5 pt),
		PaddingTop(5 pt),
		PaddingRight(5 pt),
		PaddingBottom(5 pt),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		PaddingLeft(5 pt),
		PaddingTop(5 pt),
		PaddingRight(5 pt),
		PaddingBottom(5 pt),
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

	assert_eq!(5.0, child_0_layout.left);
	assert_eq!(5.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(55.0, child_1_layout.left);
	assert_eq!(40.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(20.0, child_1_layout.height);

	assert_eq!(5.0, child_1_child_0_layout.left);
	assert_eq!(5.0, child_1_child_0_layout.top);
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

	assert_eq!(45.0, child_0_layout.left);
	assert_eq!(5.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(-5.0, child_1_layout.left);
	assert_eq!(40.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(20.0, child_1_layout.height);

	assert_eq!(-5.0, child_1_child_0_layout.left);
	assert_eq!(5.0, child_1_child_0_layout.top);
	assert_eq!(50.0, child_1_child_0_layout.width);
	assert_eq!(10.0, child_1_child_0_layout.height);
}

#[test]
fn test_align_baseline_multiline() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignItems(Align::Baseline),
		FlexWrap(Wrap::Wrap),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(50 pt),
		Height(20 pt)
	);

	let mut root_child_1_child_0 = Node::new();

	style!(root_child_1_child_0,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(50 pt),
		Height(20 pt)
	);

	let mut root_child_2_child_0 = Node::new();

	style!(root_child_2_child_0,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		Width(50 pt),
		Height(50 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);

	root_child_1.insert_child(&mut root_child_1_child_0, 0);
	root_child_2.insert_child(&mut root_child_2_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_1_child_0_layout = root_child_1_child_0.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_2_child_0_layout = root_child_2_child_0.get_layout();
	let child_3_layout = root_child_3.get_layout();

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

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(100.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(20.0, child_2_layout.height);

	assert_eq!(0.0, child_2_child_0_layout.left);
	assert_eq!(0.0, child_2_child_0_layout.top);
	assert_eq!(50.0, child_2_child_0_layout.width);
	assert_eq!(10.0, child_2_child_0_layout.height);

	assert_eq!(50.0, child_3_layout.left);
	assert_eq!(60.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(50.0, child_3_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_1_child_0_layout = root_child_1_child_0.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_2_child_0_layout = root_child_2_child_0.get_layout();
	let child_3_layout = root_child_3.get_layout();

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

	assert_eq!(50.0, child_2_layout.left);
	assert_eq!(100.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(20.0, child_2_layout.height);

	assert_eq!(0.0, child_2_child_0_layout.left);
	assert_eq!(0.0, child_2_child_0_layout.top);
	assert_eq!(50.0, child_2_child_0_layout.width);
	assert_eq!(10.0, child_2_child_0_layout.height);

	assert_eq!(0.0, child_3_layout.left);
	assert_eq!(60.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(50.0, child_3_layout.height);
}

#[test]
fn test_align_baseline_multiline_column() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::Baseline),
		FlexWrap(Wrap::Wrap),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(30 pt),
		Height(50 pt)
	);

	let mut root_child_1_child_0 = Node::new();

	style!(root_child_1_child_0,
		Width(20 pt),
		Height(20 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(40 pt),
		Height(70 pt)
	);

	let mut root_child_2_child_0 = Node::new();

	style!(root_child_2_child_0,
		Width(10 pt),
		Height(10 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		Width(50 pt),
		Height(20 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);

	root_child_1.insert_child(&mut root_child_1_child_0, 0);
	root_child_2.insert_child(&mut root_child_2_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_1_child_0_layout = root_child_1_child_0.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_2_child_0_layout = root_child_2_child_0.get_layout();
	let child_3_layout = root_child_3.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(50.0, child_1_layout.top);
	assert_eq!(30.0, child_1_layout.width);
	assert_eq!(50.0, child_1_layout.height);

	assert_eq!(0.0, child_1_child_0_layout.left);
	assert_eq!(0.0, child_1_child_0_layout.top);
	assert_eq!(20.0, child_1_child_0_layout.width);
	assert_eq!(20.0, child_1_child_0_layout.height);

	assert_eq!(50.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(40.0, child_2_layout.width);
	assert_eq!(70.0, child_2_layout.height);

	assert_eq!(0.0, child_2_child_0_layout.left);
	assert_eq!(0.0, child_2_child_0_layout.top);
	assert_eq!(10.0, child_2_child_0_layout.width);
	assert_eq!(10.0, child_2_child_0_layout.height);

	assert_eq!(50.0, child_3_layout.left);
	assert_eq!(70.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(20.0, child_3_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_1_child_0_layout = root_child_1_child_0.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_2_child_0_layout = root_child_2_child_0.get_layout();
	let child_3_layout = root_child_3.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(50.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(70.0, child_1_layout.left);
	assert_eq!(50.0, child_1_layout.top);
	assert_eq!(30.0, child_1_layout.width);
	assert_eq!(50.0, child_1_layout.height);

	assert_eq!(10.0, child_1_child_0_layout.left);
	assert_eq!(0.0, child_1_child_0_layout.top);
	assert_eq!(20.0, child_1_child_0_layout.width);
	assert_eq!(20.0, child_1_child_0_layout.height);

	assert_eq!(10.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(40.0, child_2_layout.width);
	assert_eq!(70.0, child_2_layout.height);

	assert_eq!(30.0, child_2_child_0_layout.left);
	assert_eq!(0.0, child_2_child_0_layout.top);
	assert_eq!(10.0, child_2_child_0_layout.width);
	assert_eq!(10.0, child_2_child_0_layout.height);

	assert_eq!(0.0, child_3_layout.left);
	assert_eq!(70.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(20.0, child_3_layout.height);
}

#[test]
fn test_align_baseline_multiline_column2() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::Baseline),
		FlexWrap(Wrap::Wrap),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(30 pt),
		Height(50 pt)
	);

	let mut root_child_1_child_0 = Node::new();

	style!(root_child_1_child_0,
		Width(20 pt),
		Height(20 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(40 pt),
		Height(70 pt)
	);

	let mut root_child_2_child_0 = Node::new();

	style!(root_child_2_child_0,
		Width(10 pt),
		Height(10 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		Width(50 pt),
		Height(20 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);

	root_child_1.insert_child(&mut root_child_1_child_0, 0);
	root_child_2.insert_child(&mut root_child_2_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_1_child_0_layout = root_child_1_child_0.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_2_child_0_layout = root_child_2_child_0.get_layout();
	let child_3_layout = root_child_3.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(50.0, child_1_layout.top);
	assert_eq!(30.0, child_1_layout.width);
	assert_eq!(50.0, child_1_layout.height);

	assert_eq!(0.0, child_1_child_0_layout.left);
	assert_eq!(0.0, child_1_child_0_layout.top);
	assert_eq!(20.0, child_1_child_0_layout.width);
	assert_eq!(20.0, child_1_child_0_layout.height);

	assert_eq!(50.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(40.0, child_2_layout.width);
	assert_eq!(70.0, child_2_layout.height);

	assert_eq!(0.0, child_2_child_0_layout.left);
	assert_eq!(0.0, child_2_child_0_layout.top);
	assert_eq!(10.0, child_2_child_0_layout.width);
	assert_eq!(10.0, child_2_child_0_layout.height);

	assert_eq!(50.0, child_3_layout.left);
	assert_eq!(70.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(20.0, child_3_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_1_child_0_layout = root_child_1_child_0.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_2_child_0_layout = root_child_2_child_0.get_layout();
	let child_3_layout = root_child_3.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(50.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(70.0, child_1_layout.left);
	assert_eq!(50.0, child_1_layout.top);
	assert_eq!(30.0, child_1_layout.width);
	assert_eq!(50.0, child_1_layout.height);

	assert_eq!(10.0, child_1_child_0_layout.left);
	assert_eq!(0.0, child_1_child_0_layout.top);
	assert_eq!(20.0, child_1_child_0_layout.width);
	assert_eq!(20.0, child_1_child_0_layout.height);

	assert_eq!(10.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(40.0, child_2_layout.width);
	assert_eq!(70.0, child_2_layout.height);

	assert_eq!(30.0, child_2_child_0_layout.left);
	assert_eq!(0.0, child_2_child_0_layout.top);
	assert_eq!(10.0, child_2_child_0_layout.width);
	assert_eq!(10.0, child_2_child_0_layout.height);

	assert_eq!(0.0, child_3_layout.left);
	assert_eq!(70.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(20.0, child_3_layout.height);
}

#[test]
fn test_align_baseline_multiline_row_and_column() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignItems(Align::Baseline),
		FlexWrap(Wrap::Wrap),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(50 pt),
		Height(50 pt)
	);

	let mut root_child_1_child_0 = Node::new();

	style!(root_child_1_child_0,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(50 pt),
		Height(20 pt)
	);

	let mut root_child_2_child_0 = Node::new();

	style!(root_child_2_child_0,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		Width(50 pt),
		Height(20 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);

	root_child_1.insert_child(&mut root_child_1_child_0, 0);
	root_child_2.insert_child(&mut root_child_2_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_1_child_0_layout = root_child_1_child_0.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_2_child_0_layout = root_child_2_child_0.get_layout();
	let child_3_layout = root_child_3.get_layout();

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
	assert_eq!(50.0, child_1_layout.height);

	assert_eq!(0.0, child_1_child_0_layout.left);
	assert_eq!(0.0, child_1_child_0_layout.top);
	assert_eq!(50.0, child_1_child_0_layout.width);
	assert_eq!(10.0, child_1_child_0_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(100.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(20.0, child_2_layout.height);

	assert_eq!(0.0, child_2_child_0_layout.left);
	assert_eq!(0.0, child_2_child_0_layout.top);
	assert_eq!(50.0, child_2_child_0_layout.width);
	assert_eq!(10.0, child_2_child_0_layout.height);

	assert_eq!(50.0, child_3_layout.left);
	assert_eq!(90.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(20.0, child_3_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_1_child_0_layout = root_child_1_child_0.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_2_child_0_layout = root_child_2_child_0.get_layout();
	let child_3_layout = root_child_3.get_layout();

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
	assert_eq!(50.0, child_1_layout.height);

	assert_eq!(0.0, child_1_child_0_layout.left);
	assert_eq!(0.0, child_1_child_0_layout.top);
	assert_eq!(50.0, child_1_child_0_layout.width);
	assert_eq!(10.0, child_1_child_0_layout.height);

	assert_eq!(50.0, child_2_layout.left);
	assert_eq!(100.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(20.0, child_2_layout.height);

	assert_eq!(0.0, child_2_child_0_layout.left);
	assert_eq!(0.0, child_2_child_0_layout.top);
	assert_eq!(50.0, child_2_child_0_layout.width);
	assert_eq!(10.0, child_2_child_0_layout.height);

	assert_eq!(0.0, child_3_layout.left);
	assert_eq!(90.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(20.0, child_3_layout.height);
}

#[test]
fn test_align_items_center_child_with_margin_bigger_than_parent() {
	let mut root = Node::new();

	style!(root,
		JustifyContent(Justify::Center),
		AlignItems(Align::Center),
		Width(52 pt),
		Height(52 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		AlignItems(Align::Center)
	);

	let mut root_child_0_child_0 = Node::new();

	style!(root_child_0_child_0,
		MarginLeft(10 pt),
		MarginRight(10 pt),
		Width(52 pt),
		Height(52 pt)
	);

	root.insert_child(&mut root_child_0, 0);

	root_child_0.insert_child(&mut root_child_0_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(52.0, root_layout.width);
	assert_eq!(52.0, root_layout.height);

	assert_eq!(-10.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(72.0, child_0_layout.width);
	assert_eq!(52.0, child_0_layout.height);

	assert_eq!(10.0, child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_layout.top);
	assert_eq!(52.0, child_0_child_0_layout.width);
	assert_eq!(52.0, child_0_child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(52.0, root_layout.width);
	assert_eq!(52.0, root_layout.height);

	assert_eq!(-10.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(72.0, child_0_layout.width);
	assert_eq!(52.0, child_0_layout.height);

	assert_eq!(10.0, child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_layout.top);
	assert_eq!(52.0, child_0_child_0_layout.width);
	assert_eq!(52.0, child_0_child_0_layout.height);
}

#[test]
fn test_align_items_flex_end_child_with_margin_bigger_than_parent() {
	let mut root = Node::new();

	style!(root,
		JustifyContent(Justify::Center),
		AlignItems(Align::Center),
		Width(52 pt),
		Height(52 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		AlignItems(Align::FlexEnd)
	);

	let mut root_child_0_child_0 = Node::new();

	style!(root_child_0_child_0,
		MarginLeft(10 pt),
		MarginRight(10 pt),
		Width(52 pt),
		Height(52 pt)
	);

	root.insert_child(&mut root_child_0, 0);

	root_child_0.insert_child(&mut root_child_0_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(52.0, root_layout.width);
	assert_eq!(52.0, root_layout.height);

	assert_eq!(-10.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(72.0, child_0_layout.width);
	assert_eq!(52.0, child_0_layout.height);

	assert_eq!(10.0, child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_layout.top);
	assert_eq!(52.0, child_0_child_0_layout.width);
	assert_eq!(52.0, child_0_child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(52.0, root_layout.width);
	assert_eq!(52.0, root_layout.height);

	assert_eq!(-10.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(72.0, child_0_layout.width);
	assert_eq!(52.0, child_0_layout.height);

	assert_eq!(10.0, child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_layout.top);
	assert_eq!(52.0, child_0_child_0_layout.width);
	assert_eq!(52.0, child_0_child_0_layout.height);
}

#[test]
fn test_align_items_center_child_without_margin_bigger_than_parent() {
	let mut root = Node::new();

	style!(root,
		JustifyContent(Justify::Center),
		AlignItems(Align::Center),
		Width(52 pt),
		Height(52 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		AlignItems(Align::Center)
	);

	let mut root_child_0_child_0 = Node::new();

	style!(root_child_0_child_0,
		Width(72 pt),
		Height(72 pt)
	);

	root.insert_child(&mut root_child_0, 0);

	root_child_0.insert_child(&mut root_child_0_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(52.0, root_layout.width);
	assert_eq!(52.0, root_layout.height);

	assert_eq!(-10.0, child_0_layout.left);
	assert_eq!(-10.0, child_0_layout.top);
	assert_eq!(72.0, child_0_layout.width);
	assert_eq!(72.0, child_0_layout.height);

	assert_eq!(0.0, child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_layout.top);
	assert_eq!(72.0, child_0_child_0_layout.width);
	assert_eq!(72.0, child_0_child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(52.0, root_layout.width);
	assert_eq!(52.0, root_layout.height);

	assert_eq!(-10.0, child_0_layout.left);
	assert_eq!(-10.0, child_0_layout.top);
	assert_eq!(72.0, child_0_layout.width);
	assert_eq!(72.0, child_0_layout.height);

	assert_eq!(0.0, child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_layout.top);
	assert_eq!(72.0, child_0_child_0_layout.width);
	assert_eq!(72.0, child_0_child_0_layout.height);
}

#[test]
fn test_align_items_flex_end_child_without_margin_bigger_than_parent() {
	let mut root = Node::new();

	style!(root,
		JustifyContent(Justify::Center),
		AlignItems(Align::Center),
		Width(52 pt),
		Height(52 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		AlignItems(Align::FlexEnd)
	);

	let mut root_child_0_child_0 = Node::new();

	style!(root_child_0_child_0,
		Width(72 pt),
		Height(72 pt)
	);

	root.insert_child(&mut root_child_0, 0);

	root_child_0.insert_child(&mut root_child_0_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(52.0, root_layout.width);
	assert_eq!(52.0, root_layout.height);

	assert_eq!(-10.0, child_0_layout.left);
	assert_eq!(-10.0, child_0_layout.top);
	assert_eq!(72.0, child_0_layout.width);
	assert_eq!(72.0, child_0_layout.height);

	assert_eq!(0.0, child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_layout.top);
	assert_eq!(72.0, child_0_child_0_layout.width);
	assert_eq!(72.0, child_0_child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(52.0, root_layout.width);
	assert_eq!(52.0, root_layout.height);

	assert_eq!(-10.0, child_0_layout.left);
	assert_eq!(-10.0, child_0_layout.top);
	assert_eq!(72.0, child_0_layout.width);
	assert_eq!(72.0, child_0_layout.height);

	assert_eq!(0.0, child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_layout.top);
	assert_eq!(72.0, child_0_child_0_layout.width);
	assert_eq!(72.0, child_0_child_0_layout.height);
}

#[test]
fn test_align_center_should_size_based_on_content() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::Center),
		MarginTop(20 pt),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		JustifyContent(Justify::Center),
		FlexShrink(1.0)
	);

	let mut root_child_0_child_0 = Node::new();

	style!(root_child_0_child_0,
		FlexGrow(1.0),
		FlexShrink(1.0)
	);

	let mut root_child_0_child_0_child_0 = Node::new();

	style!(root_child_0_child_0_child_0,
		Width(20 pt),
		Height(20 pt)
	);

	root.insert_child(&mut root_child_0, 0);

	root_child_0.insert_child(&mut root_child_0_child_0, 0);
	root_child_0_child_0.insert_child(&mut root_child_0_child_0_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0_child_0.get_layout();
	let child_0_child_0_child_0_layout = root_child_0_child_0_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(20.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(40.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(20.0, child_0_layout.width);
	assert_eq!(20.0, child_0_layout.height);

	assert_eq!(0.0, child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_layout.top);
	assert_eq!(20.0, child_0_child_0_layout.width);
	assert_eq!(20.0, child_0_child_0_layout.height);

	assert_eq!(0.0, child_0_child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_child_0_layout.top);
	assert_eq!(20.0, child_0_child_0_child_0_layout.width);
	assert_eq!(20.0, child_0_child_0_child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0_child_0.get_layout();
	let child_0_child_0_child_0_layout = root_child_0_child_0_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(20.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(40.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(20.0, child_0_layout.width);
	assert_eq!(20.0, child_0_layout.height);

	assert_eq!(0.0, child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_layout.top);
	assert_eq!(20.0, child_0_child_0_layout.width);
	assert_eq!(20.0, child_0_child_0_layout.height);

	assert_eq!(0.0, child_0_child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_child_0_layout.top);
	assert_eq!(20.0, child_0_child_0_child_0_layout.width);
	assert_eq!(20.0, child_0_child_0_child_0_layout.height);
}

#[test]
fn test_align_strech_should_size_based_on_parent() {
	let mut root = Node::new();

	style!(root,
		MarginTop(20 pt),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		JustifyContent(Justify::Center),
		FlexShrink(1.0)
	);

	let mut root_child_0_child_0 = Node::new();

	style!(root_child_0_child_0,
		FlexGrow(1.0),
		FlexShrink(1.0)
	);

	let mut root_child_0_child_0_child_0 = Node::new();

	style!(root_child_0_child_0_child_0,
		Width(20 pt),
		Height(20 pt)
	);

	root.insert_child(&mut root_child_0, 0);

	root_child_0.insert_child(&mut root_child_0_child_0, 0);
	root_child_0_child_0.insert_child(&mut root_child_0_child_0_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0_child_0.get_layout();
	let child_0_child_0_child_0_layout = root_child_0_child_0_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(20.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(100.0, child_0_layout.width);
	assert_eq!(20.0, child_0_layout.height);

	assert_eq!(0.0, child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_layout.top);
	assert_eq!(100.0, child_0_child_0_layout.width);
	assert_eq!(20.0, child_0_child_0_layout.height);

	assert_eq!(0.0, child_0_child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_child_0_layout.top);
	assert_eq!(20.0, child_0_child_0_child_0_layout.width);
	assert_eq!(20.0, child_0_child_0_child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0_child_0.get_layout();
	let child_0_child_0_child_0_layout = root_child_0_child_0_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(20.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(100.0, child_0_layout.width);
	assert_eq!(20.0, child_0_layout.height);

	assert_eq!(0.0, child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_layout.top);
	assert_eq!(100.0, child_0_child_0_layout.width);
	assert_eq!(20.0, child_0_child_0_layout.height);

	assert_eq!(80.0, child_0_child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_child_0_layout.top);
	assert_eq!(20.0, child_0_child_0_child_0_layout.width);
	assert_eq!(20.0, child_0_child_0_child_0_layout.height);
}
