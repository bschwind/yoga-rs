#[macro_use]
extern crate yoga;

use yoga::{Align, Direction, FlexDirection, Justify, Overflow, Node, Point, Percent, PositionType, Undefined, Wrap};
use yoga::FlexStyle::*;

#[test]
fn test_absolute_layout_width_height_start_top() {

	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		Start(10 pt),
		Top(10 pt),
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

	assert_eq!(10.0, child_layout.left);
	assert_eq!(10.0, child_layout.top);
	assert_eq!(10.0, child_layout.width);
	assert_eq!(10.0, child_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(80.0, child_layout.left);
	assert_eq!(10.0, child_layout.top);
	assert_eq!(10.0, child_layout.width);
	assert_eq!(10.0, child_layout.height);
}

#[test]
fn test_absolute_layout_width_height_end_bottom() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		End(10 pt),
		Bottom(10 pt),
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

	assert_eq!(80.0, child_layout.left);
	assert_eq!(80.0, child_layout.top);
	assert_eq!(10.0, child_layout.width);
	assert_eq!(10.0, child_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(10.0, child_layout.left);
	assert_eq!(80.0, child_layout.top);
	assert_eq!(10.0, child_layout.width);
	assert_eq!(10.0, child_layout.height);
}

#[test]
fn test_absolute_layout_start_top_end_bottom() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		Start(10 pt),
		Top(10 pt),
		End(10 pt),
		Bottom(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(10.0, child_layout.left);
	assert_eq!(10.0, child_layout.top);
	assert_eq!(80.0, child_layout.width);
	assert_eq!(80.0, child_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(10.0, child_layout.left);
	assert_eq!(10.0, child_layout.top);
	assert_eq!(80.0, child_layout.width);
	assert_eq!(80.0, child_layout.height);
}

#[test]
fn test_absolute_layout_width_height_start_top_end_bottom() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		Start(10 pt),
		Top(10 pt),
		End(10 pt),
		Bottom(10 pt),
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

	assert_eq!(10.0, child_layout.left);
	assert_eq!(10.0, child_layout.top);
	assert_eq!(10.0, child_layout.width);
	assert_eq!(10.0, child_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(80.0, child_layout.left);
	assert_eq!(10.0, child_layout.top);
	assert_eq!(10.0, child_layout.width);
	assert_eq!(10.0, child_layout.height);
}

#[test]
fn test_do_not_clamp_height_of_absolute_node_to_height_of_its_overflow_hidden_parent() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		Overflow(Overflow::Hidden),
		Width(50 pt),
		Height(50 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		Start(0 pt),
		Top(0 pt)
	);

	let mut root_child_0_child0 = Node::new();

	style!(root_child_0_child0,
		Width(100 pt),
		Height(100 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root_child_0.insert_child(&mut root_child_0_child0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();
	let child_child_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(50.0, root_layout.width);
	assert_eq!(50.0, root_layout.height);

	assert_eq!(0.0, child_layout.left);
	assert_eq!(0.0, child_layout.top);
	assert_eq!(100.0, child_layout.width);
	assert_eq!(100.0, child_layout.height);

	assert_eq!(0.0, child_child_layout.left);
	assert_eq!(0.0, child_child_layout.top);
	assert_eq!(100.0, child_child_layout.width);
	assert_eq!(100.0, child_child_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_layout = root_child_0.get_layout();
	let child_child_layout = root_child_0_child0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(50.0, root_layout.width);
	assert_eq!(50.0, root_layout.height);

	assert_eq!(-50.0, child_layout.left);
	assert_eq!(0.0, child_layout.top);
	assert_eq!(100.0, child_layout.width);
	assert_eq!(100.0, child_layout.height);

	assert_eq!(0.0, child_child_layout.left);
	assert_eq!(0.0, child_child_layout.top);
	assert_eq!(100.0, child_child_layout.width);
	assert_eq!(100.0, child_child_layout.height);
}

#[test]
fn test_absolute_layout_within_border() {
	let mut root = Node::new();

	style!(root,
		MarginLeft(10 pt),
		MarginTop(10 pt),
		MarginRight(10 pt),
		MarginBottom(10 pt),
		PaddingLeft(10 pt),
		PaddingTop(10 pt),
		PaddingRight(10 pt),
		PaddingBottom(10 pt),
		BorderLeft(10.0),
		BorderTop(10.0),
		BorderRight(10.0),
		BorderBottom(10.0),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		Left(0 pt),
		Top(0 pt),
		Width(50 pt),
		Height(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Position(PositionType::Absolute),
		Right(0 pt),
		Bottom(0 pt),
		Width(50 pt),
		Height(50 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Position(PositionType::Absolute),
		Left(0 pt),
		Top(0 pt),
		MarginLeft(10 pt),
		MarginTop(10 pt),
		MarginRight(10 pt),
		MarginBottom(10 pt),
		Width(50 pt),
		Height(50 pt)
	);

	let mut root_child_3 = Node::new();

	style!(root_child_3,
		Position(PositionType::Absolute),
		Right(0 pt),
		Bottom(0 pt),
		MarginLeft(10 pt),
		MarginTop(10 pt),
		MarginRight(10 pt),
		MarginBottom(10 pt),
		Width(50 pt),
		Height(50 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.insert_child(&mut root_child_2, 2);
	root.insert_child(&mut root_child_3, 3);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();

	assert_eq!(10.0, root_layout.left);
	assert_eq!(10.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(10.0, child_0_layout.left);
	assert_eq!(10.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(40.0, child_1_layout.left);
	assert_eq!(40.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(50.0, child_1_layout.height);

	assert_eq!(20.0, child_2_layout.left);
	assert_eq!(20.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(50.0, child_2_layout.height);

	assert_eq!(30.0, child_3_layout.left);
	assert_eq!(30.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(50.0, child_3_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();
	let child_3_layout = root_child_3.get_layout();

	assert_eq!(10.0, root_layout.left);
	assert_eq!(10.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(10.0, child_0_layout.left);
	assert_eq!(10.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(40.0, child_1_layout.left);
	assert_eq!(40.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(50.0, child_1_layout.height);

	assert_eq!(20.0, child_2_layout.left);
	assert_eq!(20.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(50.0, child_2_layout.height);

	assert_eq!(30.0, child_3_layout.left);
	assert_eq!(30.0, child_3_layout.top);
	assert_eq!(50.0, child_3_layout.width);
	assert_eq!(50.0, child_3_layout.height);
}

#[test]
fn test_absolute_layout_align_items_and_justify_content_center() {
	let mut root = Node::new();

	style!(root,
		JustifyContent(Justify::Center),
		AlignItems(Align::Center),
		FlexGrow(1.0),
		Width(110 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		Width(60 pt),
		Height(40 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(110.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(25.0, child_0_layout.left);
	assert_eq!(30.0, child_0_layout.top);
	assert_eq!(60.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(110.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(25.0, child_0_layout.left);
	assert_eq!(30.0, child_0_layout.top);
	assert_eq!(60.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);
}

#[test]
fn test_absolute_layout_align_items_and_justify_content_flex_end() {
	let mut root = Node::new();

	style!(root,
		JustifyContent(Justify::FlexEnd),
		AlignItems(Align::FlexEnd),
		FlexGrow(1.0),
		Width(110 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		Width(60 pt),
		Height(40 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(110.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(50.0, child_0_layout.left);
	assert_eq!(60.0, child_0_layout.top);
	assert_eq!(60.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(110.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(60.0, child_0_layout.top);
	assert_eq!(60.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);
}

#[test]
fn test_absolute_layout_justify_content_center() {
	let mut root = Node::new();

	style!(root,
		JustifyContent(Justify::Center),
		FlexGrow(1.0),
		Width(110 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		Width(60 pt),
		Height(40 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(110.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(30.0, child_0_layout.top);
	assert_eq!(60.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(110.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(50.0, child_0_layout.left);
	assert_eq!(30.0, child_0_layout.top);
	assert_eq!(60.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);
}

#[test]
fn test_absolute_layout_align_items_center() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::Center),
		FlexGrow(1.0),
		Width(110 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		Width(60 pt),
		Height(40 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(110.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(25.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(60.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(110.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(25.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(60.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);
}

#[test]
fn test_absolute_layout_align_items_center_on_child_only() {
	let mut root = Node::new();

	style!(root,
		FlexGrow(1.0),
		Width(110 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		AlignSelf(Align::Center),
		Position(PositionType::Absolute),
		Width(60 pt),
		Height(40 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(110.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(25.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(60.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(110.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(25.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(60.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);
}

#[test]
fn test_absolute_layout_align_items_and_justify_content_center_and_top_position() {
	let mut root = Node::new();

	style!(root,
		JustifyContent(Justify::Center),
		AlignItems(Align::Center),
		FlexGrow(1.0),
		Width(110 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		Top(10 pt),
		Width(60 pt),
		Height(40 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(110.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(25.0, child_0_layout.left);
	assert_eq!(10.0, child_0_layout.top);
	assert_eq!(60.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(110.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(25.0, child_0_layout.left);
	assert_eq!(10.0, child_0_layout.top);
	assert_eq!(60.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);
}

#[test]
fn test_absolute_layout_align_items_and_justify_content_center_and_bottom_position() {
	let mut root = Node::new();

	style!(root,
		JustifyContent(Justify::Center),
		AlignItems(Align::Center),
		FlexGrow(1.0),
		Width(110 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		Bottom(10 pt),
		Width(60 pt),
		Height(40 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(110.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(25.0, child_0_layout.left);
	assert_eq!(50.0, child_0_layout.top);
	assert_eq!(60.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(110.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(25.0, child_0_layout.left);
	assert_eq!(50.0, child_0_layout.top);
	assert_eq!(60.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);
}

#[test]
fn test_absolute_layout_align_items_and_justify_content_center_and_left_position() {
	let mut root = Node::new();

	style!(root,
		JustifyContent(Justify::Center),
		AlignItems(Align::Center),
		FlexGrow(1.0),
		Width(110 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		Left(5 pt),
		Width(60 pt),
		Height(40 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(110.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(5.0, child_0_layout.left);
	assert_eq!(30.0, child_0_layout.top);
	assert_eq!(60.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(110.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(5.0, child_0_layout.left);
	assert_eq!(30.0, child_0_layout.top);
	assert_eq!(60.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);
}

#[test]
fn test_absolute_layout_align_items_and_justify_content_center_and_right_position() {
	let mut root = Node::new();

	style!(root,
		JustifyContent(Justify::Center),
		AlignItems(Align::Center),
		FlexGrow(1.0),
		Width(110 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		Right(5 pt),
		Width(60 pt),
		Height(40 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(110.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(45.0, child_0_layout.left);
	assert_eq!(30.0, child_0_layout.top);
	assert_eq!(60.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(110.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(45.0, child_0_layout.left);
	assert_eq!(30.0, child_0_layout.top);
	assert_eq!(60.0, child_0_layout.width);
	assert_eq!(40.0, child_0_layout.height);
}

#[test]
fn test_position_root_with_rtl_should_position_withoutdirection() {
	let mut root = Node::new();

	style!(root,
		Left(72 pt),
		Width(52 pt),
		Height(52 pt)
	);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();

	assert_eq!(72.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(52.0, root_layout.width);
	assert_eq!(52.0, root_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();

	assert_eq!(72.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(52.0, root_layout.width);
	assert_eq!(52.0, root_layout.height);
}

#[test]
fn test_absolute_layout_percentage_bottom_based_on_parent_height() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(200 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		Top(50 %),
		Width(10 pt),
		Height(10 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Position(PositionType::Absolute),
		Bottom(50 %),
		Width(10 pt),
		Height(10 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Position(PositionType::Absolute),
		Top(10 %),
		Bottom(10 %),
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
	assert_eq!(200.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(100.0, child_0_layout.top);
	assert_eq!(10.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(90.0, child_1_layout.top);
	assert_eq!(10.0, child_1_layout.width);
	assert_eq!(10.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(20.0, child_2_layout.top);
	assert_eq!(10.0, child_2_layout.width);
	assert_eq!(160.0, child_2_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(200.0, root_layout.height);

	assert_eq!(90.0, child_0_layout.left);
	assert_eq!(100.0, child_0_layout.top);
	assert_eq!(10.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);

	assert_eq!(90.0, child_1_layout.left);
	assert_eq!(90.0, child_1_layout.top);
	assert_eq!(10.0, child_1_layout.width);
	assert_eq!(10.0, child_1_layout.height);

	assert_eq!(90.0, child_2_layout.left);
	assert_eq!(20.0, child_2_layout.top);
	assert_eq!(10.0, child_2_layout.width);
	assert_eq!(160.0, child_2_layout.height);
}

#[test]
fn test_absolute_layout_in_wrap_reverse_column_container() {
	let mut root = Node::new();

	style!(root,
		FlexWrap(Wrap::WrapReverse),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		Width(20 pt),
		Height(20 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(80.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(20.0, child_0_layout.width);
	assert_eq!(20.0, child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(20.0, child_0_layout.width);
	assert_eq!(20.0, child_0_layout.height);
}

#[test]
fn test_absolute_layout_in_wrap_reverse_row_container() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		FlexWrap(Wrap::WrapReverse),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		Width(20 pt),
		Height(20 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(80.0, child_0_layout.top);
	assert_eq!(20.0, child_0_layout.width);
	assert_eq!(20.0, child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(80.0, child_0_layout.left);
	assert_eq!(80.0, child_0_layout.top);
	assert_eq!(20.0, child_0_layout.width);
	assert_eq!(20.0, child_0_layout.height);
}

#[test]
fn test_absolute_layout_in_wrap_reverse_column_container_flex_end() {
	let mut root = Node::new();

	style!(root,
		FlexWrap(Wrap::WrapReverse),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		AlignSelf(Align::FlexEnd),
		Position(PositionType::Absolute),
		Width(20 pt),
		Height(20 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(20.0, child_0_layout.width);
	assert_eq!(20.0, child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(80.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(20.0, child_0_layout.width);
	assert_eq!(20.0, child_0_layout.height);
}

#[test]
fn test_absolute_layout_in_wrap_reverse_row_container_flex_end() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		FlexWrap(Wrap::WrapReverse),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		AlignSelf(Align::FlexEnd),
		Position(PositionType::Absolute),
		Width(20 pt),
		Height(20 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(20.0, child_0_layout.width);
	assert_eq!(20.0, child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(80.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(20.0, child_0_layout.width);
	assert_eq!(20.0, child_0_layout.height);
}
