extern crate ordered_float;
#[macro_use]
extern crate yoga;

use yoga::{Align, Direction, Justify, Node, Undefined};
use yoga::prelude::*;

#[test]
fn test_border_no_size() {
	let mut root = Node::new();

	style!(
		root,
		BorderLeft(10.0),
		BorderTop(10.0),
		BorderRight(10.0),
		BorderBottom(10.0)
	);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(20.0, root_layout.width);
	assert_eq!(20.0, root_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(20.0, root_layout.width);
	assert_eq!(20.0, root_layout.height);
}

#[test]
fn test_border_container_match_child() {
	let mut root = Node::new();

	style!(
		root,
		BorderLeft(10.0),
		BorderTop(10.0),
		BorderRight(10.0),
		BorderBottom(10.0)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(10 pt),
		Height(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(30.0, root_layout.width);
	assert_eq!(30.0, root_layout.height);

	assert_eq!(10.0, child_0_layout.left);
	assert_eq!(10.0, child_0_layout.top);
	assert_eq!(10.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(30.0, root_layout.width);
	assert_eq!(30.0, root_layout.height);

	assert_eq!(10.0, child_0_layout.left);
	assert_eq!(10.0, child_0_layout.top);
	assert_eq!(10.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);
}

#[test]
fn test_border_flex_child() {
	let mut root = Node::new();

	style!(root,
		BorderLeft(10.0),
		BorderTop(10.0),
		BorderRight(10.0),
		BorderBottom(10.0),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		FlexGrow(1.0),
		Width(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(10.0, child_0_layout.left);
	assert_eq!(10.0, child_0_layout.top);
	assert_eq!(10.0, child_0_layout.width);
	assert_eq!(80.0, child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(80.0, child_0_layout.left);
	assert_eq!(10.0, child_0_layout.top);
	assert_eq!(10.0, child_0_layout.width);
	assert_eq!(80.0, child_0_layout.height);
}

#[test]
fn test_border_stretch_child() {
	let mut root = Node::new();

	style!(root,
		BorderLeft(10.0),
		BorderTop(10.0),
		BorderRight(10.0),
		BorderBottom(10.0),
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
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(10.0, child_0_layout.left);
	assert_eq!(10.0, child_0_layout.top);
	assert_eq!(80.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(10.0, child_0_layout.left);
	assert_eq!(10.0, child_0_layout.top);
	assert_eq!(80.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);
}

#[test]
fn test_border_center_child() {
	let mut root = Node::new();

	style!(root,
		JustifyContent(Justify::Center),
		AlignItems(Align::Center),
		BorderStart(10.0),
		BorderEnd(20.0),
		BorderBottom(20.0),
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
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(40.0, child_0_layout.left);
	assert_eq!(35.0, child_0_layout.top);
	assert_eq!(10.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(100.0, root_layout.width);
	assert_eq!(100.0, root_layout.height);

	assert_eq!(50.0, child_0_layout.left);
	assert_eq!(35.0, child_0_layout.top);
	assert_eq!(10.0, child_0_layout.width);
	assert_eq!(10.0, child_0_layout.height);
}
