extern crate ordered_float;
#[macro_use]
extern crate yoga;

use yoga::{Align, Direction, FlexDirection, Justify, MeasureMode, Node, NodeRef, PositionType,
           Size, Undefined};
use yoga::prelude::*;

#[test]
fn test_aspect_ratio_cross_defined() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(50.0, child_0_layout.width());
	assert_eq!(50.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_main_defined() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Height(50 pt),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(50.0, child_0_layout.width());
	assert_eq!(50.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_both_dimensions_defined_row() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(100 pt),
		Height(50 pt),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(100.0, child_0_layout.width());
	assert_eq!(100.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_both_dimensions_defined_column() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(100 pt),
		Height(50 pt),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(50.0, child_0_layout.width());
	assert_eq!(50.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_align_stretch() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0, AspectRatio(1.0));

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(100.0, child_0_layout.width());
	assert_eq!(100.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_flex_grow() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Height(50 pt),
		FlexGrow(1.0),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(100.0, child_0_layout.width());
	assert_eq!(100.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_flex_shrink() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Height(150 pt),
		FlexShrink(1.0),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(100.0, child_0_layout.width());
	assert_eq!(100.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_basis() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		FlexBasis(50 pt),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(50.0, child_0_layout.width());
	assert_eq!(50.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_absolute_layout_width_defined() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		Left(0 pt),
		Top(0 pt),
		Width(50 pt),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(50.0, child_0_layout.width());
	assert_eq!(50.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_absolute_layout_height_defined() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		Left(0 pt),
		Top(0 pt),
		Height(50 pt),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(50.0, child_0_layout.width());
	assert_eq!(50.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_with_max_cross_defined() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Height(50 pt),
		MaxWidth(40 pt),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(40.0, child_0_layout.width());
	assert_eq!(50.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_with_max_main_defined() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		MaxHeight(40 pt),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(40.0, child_0_layout.width());
	assert_eq!(40.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_with_min_cross_defined() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Height(30 pt),
		MinWidth(40 pt),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(40.0, child_0_layout.width());
	assert_eq!(30.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_with_min_main_defined() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(30 pt),
		MinHeight(40 pt),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(40.0, child_0_layout.width());
	assert_eq!(40.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_double_cross() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Height(50 pt),
		AspectRatio(2.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(100.0, child_0_layout.width());
	assert_eq!(50.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_half_cross() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Height(100 pt),
		AspectRatio(0.5)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(50.0, child_0_layout.width());
	assert_eq!(100.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_double_main() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		AspectRatio(0.5)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(50.0, child_0_layout.width());
	assert_eq!(100.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_half_main() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(100 pt),
		AspectRatio(2.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(100.0, child_0_layout.width());
	assert_eq!(50.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_with_measure_func() {
	extern "C" fn measure(
		_: NodeRef,
		width: f32,
		width_mode: MeasureMode,
		height: f32,
		height_mode: MeasureMode,
	) -> Size {
		let calc_width = match width_mode {
			MeasureMode::Exactly => width,
			_ => 50.0,
		};

		let calc_height = match height_mode {
			MeasureMode::Exactly => height,
			_ => 50.0,
		};

		Size {
			width: calc_width,
			height: calc_height,
		}
	}

	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();
	root_child_0.set_measure_func(Some(measure));

	style!(root_child_0, AspectRatio(1.0));

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(50.0, child_0_layout.width());
	assert_eq!(50.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_width_height_flex_grow_row() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(200 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(50 pt),
		FlexGrow(1.0),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(200.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(100.0, child_0_layout.width());
	assert_eq!(100.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_width_height_flex_grow_column() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(200 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		Height(50 pt),
		FlexGrow(1.0),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(200.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(100.0, child_0_layout.width());
	assert_eq!(100.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_height_as_flex_basis() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		FlexDirection(FlexDirection::Row),
		Width(200 pt),
		Height(200 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Height(50 pt),
		FlexGrow(1.0),
		AspectRatio(1.0)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Height(100 pt),
		FlexGrow(1.0),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(200.0, root_layout.width());
	assert_eq!(200.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(75.0, child_0_layout.width());
	assert_eq!(75.0, child_0_layout.height());

	assert_eq!(75.0, child_1_layout.left());
	assert_eq!(0.0, child_1_layout.top());
	assert_eq!(125.0, child_1_layout.width());
	assert_eq!(125.0, child_1_layout.height());
}

#[test]
fn test_aspect_ratio_width_as_flex_basis() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(200 pt),
		Height(200 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		FlexGrow(1.0),
		AspectRatio(1.0)
	);

	let mut root_child_1 = Node::new();

	style!(root_child_1,
		Width(100 pt),
		FlexGrow(1.0),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);
	root.insert_child(&mut root_child_1, 1);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();
	let child_1_layout = root_child_1.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(200.0, root_layout.width());
	assert_eq!(200.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(75.0, child_0_layout.width());
	assert_eq!(75.0, child_0_layout.height());

	assert_eq!(0.0, child_1_layout.left());
	assert_eq!(75.0, child_1_layout.top());
	assert_eq!(125.0, child_1_layout.width());
	assert_eq!(125.0, child_1_layout.height());
}

#[test]
fn test_aspect_ratio_overrides_flex_grow_row() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		FlexDirection(FlexDirection::Row),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		FlexGrow(1.0),
		AspectRatio(0.5)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(100.0, child_0_layout.width());
	assert_eq!(200.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_overrides_flex_grow_column() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Height(50 pt),
		FlexGrow(1.0),
		AspectRatio(2.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(200.0, child_0_layout.width());
	assert_eq!(100.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_left_right_absolute() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		Left(10 pt),
		Top(10 pt),
		Right(10 pt),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(10.0, child_0_layout.left());
	assert_eq!(10.0, child_0_layout.top());
	assert_eq!(80.0, child_0_layout.width());
	assert_eq!(80.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_top_bottom_absolute() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Position(PositionType::Absolute),
		Left(10 pt),
		Top(10 pt),
		Bottom(10 pt),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(10.0, child_0_layout.left());
	assert_eq!(10.0, child_0_layout.top());
	assert_eq!(80.0, child_0_layout.width());
	assert_eq!(80.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_width_overrides_align_stretch_row() {
	let mut root = Node::new();

	style!(root,
		FlexDirection(FlexDirection::Row),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(50.0, child_0_layout.width());
	assert_eq!(50.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_height_overrides_align_stretch_column() {
	let mut root = Node::new();

	style!(root,
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Height(50 pt),
		AspectRatio(1.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(50.0, child_0_layout.width());
	assert_eq!(50.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_allow_child_overflow_parent_size() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::FlexStart),
		Width(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Height(50 pt),
		AspectRatio(4.0)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(50.0, root_layout.height());

	assert_eq!(0.0, child_0_layout.left());
	assert_eq!(0.0, child_0_layout.top());
	assert_eq!(200.0, child_0_layout.width());
	assert_eq!(50.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_defined_main_with_margin() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::Center),
		JustifyContent(Justify::Center),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Height(50 pt),
		AspectRatio(1.0),
		MarginLeft(10 pt),
		MarginRight(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(25.0, child_0_layout.left());
	assert_eq!(25.0, child_0_layout.top());
	assert_eq!(50.0, child_0_layout.width());
	assert_eq!(50.0, child_0_layout.height());
}

#[test]
fn test_aspect_ratio_defined_cross_with_margin() {
	let mut root = Node::new();

	style!(root,
		AlignItems(Align::Center),
		JustifyContent(Justify::Center),
		Width(100 pt),
		Height(100 pt)
	);

	let mut root_child_0 = Node::new();

	style!(root_child_0,
		Width(50 pt),
		AspectRatio(1.0),
		MarginLeft(10 pt),
		MarginRight(10 pt)
	);

	root.insert_child(&mut root_child_0, 0);

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();
	let child_0_layout = root_child_0.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(100.0, root_layout.width());
	assert_eq!(100.0, root_layout.height());

	assert_eq!(25.0, child_0_layout.left());
	assert_eq!(25.0, child_0_layout.top());
	assert_eq!(50.0, child_0_layout.width());
	assert_eq!(50.0, child_0_layout.height());
}
