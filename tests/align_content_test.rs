extern crate ordered_float;
#[macro_use]
extern crate yoga;

use ordered_float::OrderedFloat;
use yoga::{Align, Direction, FlexDirection, Node, Percent, Point, Undefined, Wrap};
use yoga::FlexStyle::*;

#[test]
fn test_align_content_flex_start() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		FlexWrap(Wrap::Wrap),
		Width(130 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_4 = Node::new();

	style!(root_child_4,
		Width(50 pt),
		Height(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);
	root.insert_child(&mut root_child_4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(130.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(10.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(10.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(10.0, child_2_layout.height);

	assert_eq!(50.0, child_3_layout.left);
	assert_eq!(10.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(10.0, child_3_layout.height);

	assert_eq!(0.0, child_4_layout.left);
	assert_eq!(20.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(10.0, child_4_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(130.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(80.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);

	assert_eq!(30.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(10.0, child_1_layout.height);

	assert_eq!(80.0, child_2_layout.left);
	assert_eq!(10.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(10.0, child_2_layout.height);

	assert_eq!(30.0, child_3_layout.left);
	assert_eq!(10.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(10.0, child_3_layout.height);

	assert_eq!(80.0, child_4_layout.left);
	assert_eq!(20.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(10.0, child_4_layout.height);
}

#[test]
fn test_align_content_flex_start_without_height_on_children() {
	let mut root = Node::new();

	style!(root,
		FlexWrap(Wrap::Wrap),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(50 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_4 = Node::new();

	style!(root_child_4,
		Width(50 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);
	root.insert_child(&mut root_child_4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(0.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(10.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(10.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(0.0, child_2_layout.height);

	assert_eq!(0.0, child_3_layout.left);
	assert_eq!(10.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(10.0, child_3_layout.height);

	assert_eq!(0.0, child_4_layout.left);
	assert_eq!(20.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(0.0, child_4_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(50.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(0.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(10.0, child_1_layout.height);

	assert_eq!(50.0, child_2_layout.left);
	assert_eq!(10.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(0.0, child_2_layout.height);

	assert_eq!(50.0, child_3_layout.left);
	assert_eq!(10.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(10.0, child_3_layout.height);

	assert_eq!(50.0, child_4_layout.left);
	assert_eq!(20.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(0.0, child_4_layout.height);
}

#[test]
fn test_align_content_flex_start_with_flex() {
	let mut root = Node::new();

	style!(root,
		FlexWrap(Wrap::Wrap),
		Width(100 pt),
		Height(120 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		FlexGrow(1.0),
		FlexBasis(0 %),
		Width(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		FlexGrow(1.0),
		FlexBasis(0 %),
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(50 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		FlexGrow(1.0),
		FlexShrink(1.0),
		FlexBasis(0 %),
		Width(50 pt)
	);

	let mut root_child_4 = Node::new();

	style!(root_child_4,
		Width(50 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);
	root.insert_child(&mut root_child_4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(120.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(40.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(40.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(80.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(0.0, child_2_layout.height);

	assert_eq!(0.0, child_3_layout.left);
	assert_eq!(80.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(40.0, child_3_layout.height);

	assert_eq!(0.0, child_4_layout.left);
	assert_eq!(120.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(0.0, child_4_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(120.0, root_layout.height);

	assert_eq!(50.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(40.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(40.0, child_1_layout.height);

	assert_eq!(50.0, child_2_layout.left);
	assert_eq!(80.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(0.0, child_2_layout.height);

	assert_eq!(50.0, child_3_layout.left);
	assert_eq!(80.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(40.0, child_3_layout.height);

	assert_eq!(50.0, child_4_layout.left);
	assert_eq!(120.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(0.0, child_4_layout.height);
}

#[test]
fn test_align_content_flex_end() {
	let mut root = Node::new();

	style!(root,
		AlignContent(Align::FlexEnd),
		FlexWrap(Wrap::Wrap),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_4 = Node::new();

	style!(root_child_4,
		Width(50 pt),
		Height(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);
	root.insert_child(&mut root_child_4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(10.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(10.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(20.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(10.0, child_2_layout.height);

	assert_eq!(0.0, child_3_layout.left);
	assert_eq!(30.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(10.0, child_3_layout.height);

	assert_eq!(0.0, child_4_layout.left);
	assert_eq!(40.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(10.0, child_4_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(50.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(10.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(10.0, child_1_layout.height);

	assert_eq!(50.0, child_2_layout.left);
	assert_eq!(20.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(10.0, child_2_layout.height);

	assert_eq!(50.0, child_3_layout.left);
	assert_eq!(30.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(10.0, child_3_layout.height);

	assert_eq!(50.0, child_4_layout.left);
	assert_eq!(40.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(10.0, child_4_layout.height);
}

#[test]
fn test_align_content_stretch() {
	let mut root = Node::new();

	style!(root,
		AlignContent(Align::Stretch),
		FlexWrap(Wrap::Wrap),
		Width(150 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(50 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(50 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		Width(50 pt)
	);

	let mut root_child_4 = Node::new();

	style!(root_child_4,
		Width(50 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);
	root.insert_child(&mut root_child_4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(0.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(0.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(0.0, child_2_layout.height);

	assert_eq!(0.0, child_3_layout.left);
	assert_eq!(0.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(0.0, child_3_layout.height);

	assert_eq!(0.0, child_4_layout.left);
	assert_eq!(0.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(0.0, child_4_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(100.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(0.0, child_0_layout.height);

	assert_eq!(100.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(0.0, child_1_layout.height);

	assert_eq!(100.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(0.0, child_2_layout.height);

	assert_eq!(100.0, child_3_layout.left);
	assert_eq!(0.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(0.0, child_3_layout.height);

	assert_eq!(100.0, child_4_layout.left);
	assert_eq!(0.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(0.0, child_4_layout.height);
}

#[test]
fn test_align_content_spacebetween() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignContent(Align::SpaceBetween),
		FlexWrap(Wrap::Wrap),
		Width(130 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_4 = Node::new();

	style!(root_child_4,
		Width(50 pt),
		Height(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);
	root.insert_child(&mut root_child_4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(130.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(10.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(45.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(10.0, child_2_layout.height);

	assert_eq!(50.0, child_3_layout.left);
	assert_eq!(45.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(10.0, child_3_layout.height);

	assert_eq!(0.0, child_4_layout.left);
	assert_eq!(90.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(10.0, child_4_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(130.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(80.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);

	assert_eq!(30.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(10.0, child_1_layout.height);

	assert_eq!(80.0, child_2_layout.left);
	assert_eq!(45.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(10.0, child_2_layout.height);

	assert_eq!(30.0, child_3_layout.left);
	assert_eq!(45.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(10.0, child_3_layout.height);

	assert_eq!(80.0, child_4_layout.left);
	assert_eq!(90.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(10.0, child_4_layout.height);
}

#[test]
fn test_align_content_spacearound() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignContent(Align::SpaceAround),
		FlexWrap(Wrap::Wrap),
		Width(140 pt),
		Height(120 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		Width(50 pt),
		Height(10 pt)
	);

	let mut root_child_4 = Node::new();

	style!(root_child_4,
		Width(50 pt),
		Height(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);
	root.insert_child(&mut root_child_4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(140.0, root_layout.width);
	assert_eq!(120.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(15.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(15.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(10.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(55.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(10.0, child_2_layout.height);

	assert_eq!(50.0, child_3_layout.left);
	assert_eq!(55.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(10.0, child_3_layout.height);

	assert_eq!(0.0, child_4_layout.left);
	assert_eq!(95.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(10.0, child_4_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(140.0, root_layout.width);
	assert_eq!(120.0, root_layout.height);

	assert_eq!(90.0, child_0_layout.left);
	assert_eq!(15.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);

	assert_eq!(40.0, child_1_layout.left);
	assert_eq!(15.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(10.0, child_1_layout.height);

	assert_eq!(90.0, child_2_layout.left);
	assert_eq!(55.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(10.0, child_2_layout.height);

	assert_eq!(40.0, child_3_layout.left);
	assert_eq!(55.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(10.0, child_3_layout.height);

	assert_eq!(90.0, child_4_layout.left);
	assert_eq!(95.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(10.0, child_4_layout.height);
}

#[test]
fn test_align_content_stretch_row() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignContent(Align::Stretch),
		FlexWrap(Wrap::Wrap),
		Width(150 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(50 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(50 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		Width(50 pt)
	);

	let mut root_child_4 = Node::new();

	style!(root_child_4,
		Width(50 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);
	root.insert_child(&mut root_child_4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(50.0, child_1_layout.height);

	assert_eq!(100.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(50.0, child_2_layout.height);

	assert_eq!(0.0, child_3_layout.left);
	assert_eq!(50.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(50.0, child_3_layout.height);

	assert_eq!(50.0, child_4_layout.left);
	assert_eq!(50.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(50.0, child_4_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(100.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(50.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(50.0, child_2_layout.height);

	assert_eq!(100.0, child_3_layout.left);
	assert_eq!(50.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(50.0, child_3_layout.height);

	assert_eq!(50.0, child_4_layout.left);
	assert_eq!(50.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(50.0, child_4_layout.height);
}

#[test]
fn test_align_content_stretch_row_with_children() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignContent(Align::Stretch),
		FlexWrap(Wrap::Wrap),
		Width(150 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt)
	);

	let mut root_child_0_child_0 = Node::new();

	style!(root_child_0_child_0,
		FlexGrow(1.0),
		FlexShrink(1.0),
		FlexBasis(0%)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(50 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(50 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		Width(50 pt)
	);

	let mut root_child_4 = Node::new();

	style!(root_child_4,
		Width(50 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);
	root.insert_child(&mut root_child_4, 4);

	root_child_0.insert_child(&mut root_child_0_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(0.0, child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_layout.top);
	assert_eq!(50.0, child_0_child_0_layout.width);
	assert_eq!(50.0, child_0_child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(50.0, child_1_layout.height);

	assert_eq!(100.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(50.0, child_2_layout.height);

	assert_eq!(0.0, child_3_layout.left);
	assert_eq!(50.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(50.0, child_3_layout.height);

	assert_eq!(50.0, child_4_layout.left);
	assert_eq!(50.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(50.0, child_4_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(100.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(0.0, child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_layout.top);
	assert_eq!(50.0, child_0_child_0_layout.width);
	assert_eq!(50.0, child_0_child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(50.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(50.0, child_2_layout.height);

	assert_eq!(100.0, child_3_layout.left);
	assert_eq!(50.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(50.0, child_3_layout.height);

	assert_eq!(50.0, child_4_layout.left);
	assert_eq!(50.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(50.0, child_4_layout.height);
}

#[test]
fn test_align_content_stretch_row_with_flex() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignContent(Align::Stretch),
		FlexWrap(Wrap::Wrap),
		Width(150 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		FlexGrow(1.0),
		FlexShrink(1.0),
		FlexBasis(0%),
		Width(50 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(50 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		FlexGrow(1.0),
		FlexShrink(1.0),
		FlexBasis(0%),
		Width(50 pt)
	);

	let mut root_child_4 = Node::new();

	style!(root_child_4,
		Width(50 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);
	root.insert_child(&mut root_child_4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(0.0, child_1_layout.width);
	assert_eq!(100.0, child_1_layout.height);

	assert_eq!(50.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(100.0, child_2_layout.height);

	assert_eq!(100.0, child_3_layout.left);
	assert_eq!(0.0, child_3_layout.top);
	assert_eq!(0.0, child_3_layout.width);
	assert_eq!(100.0, child_3_layout.height);

	assert_eq!(100.0, child_4_layout.left);
	assert_eq!(0.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(100.0, child_4_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(100.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(100.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(0.0, child_1_layout.width);
	assert_eq!(100.0, child_1_layout.height);

	assert_eq!(50.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(100.0, child_2_layout.height);

	assert_eq!(50.0, child_3_layout.left);
	assert_eq!(0.0, child_3_layout.top);
	assert_eq!(0.0, child_3_layout.width);
	assert_eq!(100.0, child_3_layout.height);

	assert_eq!(0.0, child_4_layout.left);
	assert_eq!(0.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(100.0, child_4_layout.height);
}

#[test]

fn test_align_content_stretch_row_with_flex_no_shrink() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignContent(Align::Stretch),
		FlexWrap(Wrap::Wrap),
		Width(150 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		FlexGrow(1.0),
		FlexShrink(1.0),
		FlexBasis(0%),
		Width(50 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(50 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		FlexGrow(1.0),
		FlexBasis(0%),
		Width(50 pt)
	);

	let mut root_child_4 = Node::new();

	style!(root_child_4,
		Width(50 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);
	root.insert_child(&mut root_child_4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(0.0, child_1_layout.width);
	assert_eq!(100.0, child_1_layout.height);

	assert_eq!(50.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(100.0, child_2_layout.height);

	assert_eq!(100.0, child_3_layout.left);
	assert_eq!(0.0, child_3_layout.top);
	assert_eq!(0.0, child_3_layout.width);
	assert_eq!(100.0, child_3_layout.height);

	assert_eq!(100.0, child_4_layout.left);
	assert_eq!(0.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(100.0, child_4_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(100.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(100.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(0.0, child_1_layout.width);
	assert_eq!(100.0, child_1_layout.height);

	assert_eq!(50.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(100.0, child_2_layout.height);

	assert_eq!(50.0, child_3_layout.left);
	assert_eq!(0.0, child_3_layout.top);
	assert_eq!(0.0, child_3_layout.width);
	assert_eq!(100.0, child_3_layout.height);

	assert_eq!(0.0, child_4_layout.left);
	assert_eq!(0.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(100.0, child_4_layout.height);
}

#[test]

fn test_align_content_stretch_row_with_margin() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignContent(Align::Stretch),
		FlexWrap(Wrap::Wrap),
		Width(150 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		MarginLeft(10 pt),
		MarginTop(10 pt),
		MarginRight(10 pt),
		MarginBottom(10 pt),
		Width(50 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(50 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		MarginLeft(10 pt),
		MarginTop(10 pt),
		MarginRight(10 pt),
		MarginBottom(10 pt),
		Width(50 pt)
	);

	let mut root_child_4 = Node::new();

	style!(root_child_4,
		Width(50 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);
	root.insert_child(&mut root_child_4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);

	assert_eq!(60.0, child_1_layout.left);
	assert_eq!(10.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(20.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(40.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(40.0, child_2_layout.height);

	assert_eq!(60.0, child_3_layout.left);
	assert_eq!(50.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(20.0, child_3_layout.height);

	assert_eq!(0.0, child_4_layout.left);
	assert_eq!(80.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(20.0, child_4_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(100.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);

	assert_eq!(40.0, child_1_layout.left);
	assert_eq!(10.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(20.0, child_1_layout.height);

	assert_eq!(100.0, child_2_layout.left);
	assert_eq!(40.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(40.0, child_2_layout.height);

	assert_eq!(40.0, child_3_layout.left);
	assert_eq!(50.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(20.0, child_3_layout.height);

	assert_eq!(100.0, child_4_layout.left);
	assert_eq!(80.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(20.0, child_4_layout.height);
}

#[test]

fn test_align_content_stretch_row_with_padding() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignContent(Align::Stretch),
		FlexWrap(Wrap::Wrap),
		Width(150 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		PaddingLeft(10 pt),
		PaddingTop(10 pt),
		PaddingRight(10 pt),
		PaddingBottom(10 pt),
		Width(50 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(50 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		PaddingLeft(10 pt),
		PaddingTop(10 pt),
		PaddingRight(10 pt),
		PaddingBottom(10 pt),
		Width(50 pt)
	);

	let mut root_child_4 = Node::new();

	style!(root_child_4,
		Width(50 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);
	root.insert_child(&mut root_child_4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(50.0, child_1_layout.height);

	assert_eq!(100.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(50.0, child_2_layout.height);

	assert_eq!(0.0, child_3_layout.left);
	assert_eq!(50.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(50.0, child_3_layout.height);

	assert_eq!(50.0, child_4_layout.left);
	assert_eq!(50.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(50.0, child_4_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(100.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(50.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(50.0, child_2_layout.height);

	assert_eq!(100.0, child_3_layout.left);
	assert_eq!(50.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(50.0, child_3_layout.height);

	assert_eq!(50.0, child_4_layout.left);
	assert_eq!(50.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(50.0, child_4_layout.height);
}

#[test]
fn test_align_content_stretch_row_with_single_row() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignContent(Align::Stretch),
		FlexWrap(Wrap::Wrap),
		Width(150 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		PaddingLeft(10 pt),
		PaddingTop(10 pt),
		PaddingRight(10 pt),
		PaddingBottom(10 pt),
		Width(50 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(100.0, child_1_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(100.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(100.0, child_1_layout.height);
}

#[test]
fn test_align_content_stretch_row_with_fixed_height() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignContent(Align::Stretch),
		FlexWrap(Wrap::Wrap),
		Width(150 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(50 pt),
		Height(60 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(50 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		Width(50 pt)
	);

	let mut root_child_4 = Node::new();

	style!(root_child_4,
		Width(50 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);
	root.insert_child(&mut root_child_4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(80.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(60.0, child_1_layout.height);

	assert_eq!(100.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(80.0, child_2_layout.height);

	assert_eq!(0.0, child_3_layout.left);
	assert_eq!(80.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(20.0, child_3_layout.height);

	assert_eq!(50.0, child_4_layout.left);
	assert_eq!(80.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(20.0, child_4_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(100.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(80.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(60.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(80.0, child_2_layout.height);

	assert_eq!(100.0, child_3_layout.left);
	assert_eq!(80.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(20.0, child_3_layout.height);

	assert_eq!(50.0, child_4_layout.left);
	assert_eq!(80.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(20.0, child_4_layout.height);
}

#[test]
fn test_align_content_stretch_row_with_max_height() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignContent(Align::Stretch),
		FlexWrap(Wrap::Wrap),
		Width(150 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(50 pt),
		MaxHeight(20 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(50 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		Width(50 pt)
	);

	let mut root_child_4 = Node::new();

	style!(root_child_4,
		Width(50 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);
	root.insert_child(&mut root_child_4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(20.0, child_1_layout.height);

	assert_eq!(100.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(50.0, child_2_layout.height);

	assert_eq!(0.0, child_3_layout.left);
	assert_eq!(50.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(50.0, child_3_layout.height);

	assert_eq!(50.0, child_4_layout.left);
	assert_eq!(50.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(50.0, child_4_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(100.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(20.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(50.0, child_2_layout.height);

	assert_eq!(100.0, child_3_layout.left);
	assert_eq!(50.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(50.0, child_3_layout.height);

	assert_eq!(50.0, child_4_layout.left);
	assert_eq!(50.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(50.0, child_4_layout.height);
}

#[test]
fn test_align_content_stretch_row_with_min_height() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignContent(Align::Stretch),
		FlexWrap(Wrap::Wrap),
		Width(150 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(50 pt),
		MinHeight(80 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(50 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		Width(50 pt)
	);

	let mut root_child_4 = Node::new();

	style!(root_child_4,
		Width(50 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);
	root.insert_child(&mut root_child_4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(90.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(90.0, child_1_layout.height);

	assert_eq!(100.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(90.0, child_2_layout.height);

	assert_eq!(0.0, child_3_layout.left);
	assert_eq!(90.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(10.0, child_3_layout.height);

	assert_eq!(50.0, child_4_layout.left);
	assert_eq!(90.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(10.0, child_4_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(150.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(100.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(90.0, child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(90.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(0.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(90.0, child_2_layout.height);

	assert_eq!(100.0, child_3_layout.left);
	assert_eq!(90.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(10.0, child_3_layout.height);

	assert_eq!(50.0, child_4_layout.left);
	assert_eq!(90.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(10.0, child_4_layout.height);
}

#[test]
fn test_align_content_stretch_column() {
	let mut root = Node::new();

	style!(root,
		AlignContent(Align::Stretch),
		FlexWrap(Wrap::Wrap),
		Width(100 pt),
		Height(150 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Height(50 pt)
	);

	let mut root_child_0_child_0 = Node::new();

	style!(root_child_0_child_0,
		FlexGrow(1.0),
		FlexShrink(1.0),
		FlexBasis(0%)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		FlexGrow(1.0),
		FlexShrink(1.0),
		FlexBasis(0%),
		Height(50 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Height(50 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		Height(50 pt)
	);

	let mut root_child_4 = Node::new();

	style!(root_child_4,
		Height(50 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);
	root.insert_child(&mut root_child_4, 4);

	root_child_0.insert_child(&mut root_child_0_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(150.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(0.0, child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_layout.top);
	assert_eq!(50.0, child_0_child_0_layout.width);
	assert_eq!(50.0, child_0_child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(50.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(0.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(50.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(50.0, child_2_layout.height);

	assert_eq!(0.0, child_3_layout.left);
	assert_eq!(100.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(50.0, child_3_layout.height);

	assert_eq!(50.0, child_4_layout.left);
	assert_eq!(0.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(50.0, child_4_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();
	let child_4_layout = root_child_4.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(150.0, root_layout.height);

	assert_eq!(50.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(0.0, child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_layout.top);
	assert_eq!(50.0, child_0_child_0_layout.width);
	assert_eq!(50.0, child_0_child_0_layout.height);

	assert_eq!(50.0, child_1_layout.left);
	assert_eq!(50.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(0.0, child_1_layout.height);

	assert_eq!(50.0, child_2_layout.left);
	assert_eq!(50.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(50.0, child_2_layout.height);

	assert_eq!(50.0, child_3_layout.left);
	assert_eq!(100.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(50.0, child_3_layout.height);

	assert_eq!(0.0, child_4_layout.left);
	assert_eq!(0.0, child_4_layout.top);
	assert_eq!(50.0, child_4_layout.width);
	assert_eq!(50.0, child_4_layout.height);
}

#[test]
fn test_align_content_stretch_is_not_overriding_align_items() {
	let mut root = Node::new();

	style!(root, AlignContent(Align::Stretch));

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		FlexDirection(FlexDirection::Row),
		AlignContent(Align::Stretch),
		AlignItems(Align::Center),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0_child_0 = Node::new();

	style!(root_child_0_child_0,
		AlignContent(Align::Stretch),
		Width(10 pt),
		Height(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);

	root_child_0.insert_child(&mut root_child_0_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(100.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(0.0, child_0_child_0_layout.left);
	assert_eq!(45.0, child_0_child_0_layout.top);
	assert_eq!(10.0, child_0_child_0_layout.width);
	assert_eq!(10.0, child_0_child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_0_child_0_layout = root_child_0_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(100.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(90.0, child_0_child_0_layout.left);
	assert_eq!(45.0, child_0_child_0_layout.top);
	assert_eq!(10.0, child_0_child_0_layout.width);
	assert_eq!(10.0, child_0_child_0_layout.height);
}
