extern crate ordered_float;
#[macro_use]
extern crate yoga;

use yoga::{Direction, FlexDirection, Node, Undefined};
use yoga::prelude::*;

#[test]
fn test_flex_basis_flex_grow_column() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		FlexGrow(1.0),
		FlexBasis(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1, FlexGrow(1.0));

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
	assert_eq!(75.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(75.0, child_1_layout.top);
	assert_eq!(100.0, child_1_layout.width);
	assert_eq!(25.0, child_1_layout.height);

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
	assert_eq!(75.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(75.0, child_1_layout.top);
	assert_eq!(100.0, child_1_layout.width);
	assert_eq!(25.0, child_1_layout.height);
}

#[test]
fn test_flex_basis_flex_grow_row() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		FlexGrow(1.0),
		FlexBasis(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1, FlexGrow(1.0));

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
	assert_eq!(75.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(75.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(25.0, child_1_layout.width);
	assert_eq!(100.0, child_1_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(25.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(75.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(25.0, child_1_layout.width);
	assert_eq!(100.0, child_1_layout.height);
}

#[test]
fn test_flex_basis_flex_shrink_column() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		FlexShrink(1.0),
		FlexBasis(100 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		FlexBasis(50 pt)
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
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(50.0, child_1_layout.top);
	assert_eq!(100.0, child_1_layout.width);
	assert_eq!(50.0, child_1_layout.height);

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
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(50.0, child_1_layout.top);
	assert_eq!(100.0, child_1_layout.width);
	assert_eq!(50.0, child_1_layout.height);
}

#[test]
fn test_flex_basis_flex_shrink_row() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		FlexShrink(1.0),
		FlexBasis(100 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		FlexBasis(50 pt)
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
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(50.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(100.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(0.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(100.0, child_1_layout.height);
}

#[test]
fn test_flex_shrink_to_zero() {
	let mut root = Node::new();

	style!(root,
		Height(75 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(50 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		FlexShrink(1.0),
		Width(50 pt),
		Height(50 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		Width(50 pt),
		Height(50 pt)
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
	assert_eq!(50.0, root_layout.width);
	assert_eq!(75.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(50.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(0.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(50.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(50.0, child_2_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_2_layout = root_child_2.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(50.0, root_layout.width);
	assert_eq!(75.0, root_layout.height);

	assert_eq!(0.0, child_0_layout.left);
	assert_eq!(0.0, child_0_layout.top);
	assert_eq!(50.0, child_0_layout.width);
	assert_eq!(50.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(50.0, child_1_layout.top);
	assert_eq!(50.0, child_1_layout.width);
	assert_eq!(0.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(50.0, child_2_layout.top);
	assert_eq!(50.0, child_2_layout.width);
	assert_eq!(50.0, child_2_layout.height);
}

#[test]
fn test_flex_basis_overrides_main_size() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		FlexGrow(1.0),
		FlexBasis(50 pt),
		Height(20 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		FlexGrow(1.0),
		Height(10 pt)
	);

	let mut root_child_2 = Node::new();

	style!(root_child_2,
		FlexGrow(1.0),
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
	assert_eq!(60.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(60.0, child_1_layout.top);
	assert_eq!(100.0, child_1_layout.width);
	assert_eq!(20.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(80.0, child_2_layout.top);
	assert_eq!(100.0, child_2_layout.width);
	assert_eq!(20.0, child_2_layout.height);

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
	assert_eq!(60.0, child_0_layout.height);

	assert_eq!(0.0, child_1_layout.left);
	assert_eq!(60.0, child_1_layout.top);
	assert_eq!(100.0, child_1_layout.width);
	assert_eq!(20.0, child_1_layout.height);

	assert_eq!(0.0, child_2_layout.left);
	assert_eq!(80.0, child_2_layout.top);
	assert_eq!(100.0, child_2_layout.width);
	assert_eq!(20.0, child_2_layout.height);
}

#[test]
fn test_flex_grow_shrink_at_most() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	// No style for root_child_0

	let mut root_child_0_child_0 = Node::new();

	style!(root_child_0_child_0, FlexGrow(1.0), FlexShrink(1.0));

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
	assert_eq!(0.0, child_0_layout.height);

	assert_eq!(0.0, child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_layout.top);
	assert_eq!(100.0, child_0_child_0_layout.width);
	assert_eq!(0.0, child_0_child_0_layout.height);

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
	assert_eq!(0.0, child_0_layout.height);

	assert_eq!(0.0, child_0_child_0_layout.left);
	assert_eq!(0.0, child_0_child_0_layout.top);
	assert_eq!(100.0, child_0_child_0_layout.width);
	assert_eq!(0.0, child_0_child_0_layout.height);
}
