#[macro_use]
extern crate yoga;

use yoga::{Align, Direction, FlexDirection, Node, YGInternalNodeRef, Undefined};
use yoga::prelude::*;

#[test]
fn test_align_baseline_customer_func() {
	extern "C" fn baseline_func(_: YGInternalNodeRef, _: f32, _: f32) -> f32 {
		10.0
	}

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

	let mut root_child_1_child_0 = Node::new();
	root_child_1_child_0.set_baseline_func(Some(baseline_func));

	style!(root_child_1_child_0,
		Width(50 pt),
		Height(20 pt)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);

	root_child_1.insert_child(&mut root_child_1_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();
	let child_1_child_0_layout = root_child_1_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(50.0, child_0_layout.width());
	assert_eq!(50.0, child_0_layout.height());

	assert_eq!(50.0, child_1_layout.left());
	assert_eq!(40.0, child_1_layout.top());
	assert_eq!(50.0, child_1_layout.width());
	assert_eq!(20.0, child_1_layout.height());

	assert_eq!(0.0, child_1_child_0_layout.left());
	assert_eq!(0.0, child_1_child_0_layout.top());
	assert_eq!(50.0, child_1_child_0_layout.width());
	assert_eq!(20.0, child_1_child_0_layout.height());
}
