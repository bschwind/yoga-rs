extern crate ordered_float;
#[macro_use]
extern crate yoga;

use yoga::{Align, Direction, Display, FlexDirection, Node, Undefined};
use yoga::prelude::*;

#[test]
fn test_dirty_propagation() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(20 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(50 pt),
		Height(20 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	style!(root_child_0,
		Width(20 pt)
	);

	assert!(root_child_0.is_dirty());
	assert!(!root_child_1.is_dirty());
	assert!(root.is_dirty());

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert!(!root_child_0.is_dirty());
	assert!(!root_child_1.is_dirty());
	assert!(!root.is_dirty());
}

#[test]
fn test_dirty_propagation_only_if_prop_changed() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(20 pt)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(50 pt),
		Height(20 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	style!(root_child_0,
		Width(50 pt)
	);

	assert!(!root_child_0.is_dirty());
	assert!(!root_child_1.is_dirty());
	assert!(!root.is_dirty());
}

#[test]
fn test_dirty_mark_all_children_as_dirty_when_display_changes() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0, Display(Display::Flex), FlexGrow(1.0));

	let mut root_child_1 = Node::new();

	style!(root_child_1, Display(Display::None), FlexGrow(1.0));

	let mut root_child_1_child_0 = Node::new();
	let mut root_child_1_child_0_child_0 = Node::new();

	style!(root_child_1_child_0_child_0,
		Width(8 pt),
		Height(16 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);

	root_child_1.insert_child(&mut root_child_1_child_0, 0);
	root_child_1_child_0.insert_child(&mut root_child_1_child_0_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_child_1_child_0_child_0_layout = root_child_1_child_0_child_0.get_layout();
	assert_eq!(0.0, root_child_1_child_0_child_0_layout.width());
	assert_eq!(0.0, root_child_1_child_0_child_0_layout.height());

	root_child_0.set_display(Display::None);
	root_child_1.set_display(Display::Flex);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_child_1_child_0_child_0_layout = root_child_1_child_0_child_0.get_layout();
	assert_eq!(8.0, root_child_1_child_0_child_0_layout.width());
	assert_eq!(16.0, root_child_1_child_0_child_0_layout.height());

	root_child_0.set_display(Display::Flex);
	root_child_1.set_display(Display::None);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_child_1_child_0_child_0_layout = root_child_1_child_0_child_0.get_layout();
	assert_eq!(0.0, root_child_1_child_0_child_0_layout.width());
	assert_eq!(0.0, root_child_1_child_0_child_0_layout.height());

	root_child_0.set_display(Display::None);
	root_child_1.set_display(Display::Flex);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_child_1_child_0_child_0_layout = root_child_1_child_0_child_0.get_layout();
	assert_eq!(8.0, root_child_1_child_0_child_0_layout.width());
	assert_eq!(16.0, root_child_1_child_0_child_0_layout.height());
}

#[test]
fn test_dirty_node_only_if_children_are_actually_removed() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(25 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let mut root_child_1 = Node::new();

	root.remove_child(&mut root_child_1);

	assert!(!root.is_dirty());
	drop(root_child_1);

	root.remove_child(&mut root_child_0);
	assert!(root.is_dirty());
}
