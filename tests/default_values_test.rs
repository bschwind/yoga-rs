extern crate yoga;

use yoga::{Align, Direction, FlexDirection, Justify, Node, Overflow, PositionType, StyleUnit, Wrap};

#[test]
fn test_assert_default_values() {
	let root = Node::new();

	assert_eq!(root.get_child_count(), 0);
	assert!(root.get_child(1).is_null());

	assert_eq!(Direction::Inherit, root.get_style_direction());
	assert_eq!(FlexDirection::Column, root.get_flex_direction());
	assert_eq!(Justify::FlexStart, root.get_justify_content());
	assert_eq!(Align::FlexStart, root.get_align_content());
	assert_eq!(Align::Stretch, root.get_align_items());
	assert_eq!(Align::Auto, root.get_align_self());
	assert_eq!(PositionType::Relative, root.get_position_type());
	assert_eq!(Wrap::NoWrap, root.get_flex_wrap());
	assert_eq!(Overflow::Visible, root.get_overflow());
	assert_eq!(0.0, root.get_flex_grow());
	assert_eq!(0.0, root.get_flex_shrink());
	assert_eq!(StyleUnit::Auto, root.get_flex_basis());

	assert_eq!(StyleUnit::UndefinedValue, root.get_style_position_left());
	assert_eq!(StyleUnit::UndefinedValue, root.get_style_position_top());
	assert_eq!(StyleUnit::UndefinedValue, root.get_style_position_right());
	assert_eq!(StyleUnit::UndefinedValue, root.get_style_position_bottom());
	assert_eq!(StyleUnit::UndefinedValue, root.get_style_position_start());
	assert_eq!(StyleUnit::UndefinedValue, root.get_style_position_end());

	assert_eq!(StyleUnit::UndefinedValue, root.get_style_margin_left());
	assert_eq!(StyleUnit::UndefinedValue, root.get_style_margin_top());
	assert_eq!(StyleUnit::UndefinedValue, root.get_style_margin_right());
	assert_eq!(StyleUnit::UndefinedValue, root.get_style_margin_bottom());
	assert_eq!(StyleUnit::UndefinedValue, root.get_style_margin_start());
	assert_eq!(StyleUnit::UndefinedValue, root.get_style_margin_end());

	assert_eq!(StyleUnit::UndefinedValue, root.get_style_padding_left());
	assert_eq!(StyleUnit::UndefinedValue, root.get_style_padding_top());
	assert_eq!(StyleUnit::UndefinedValue, root.get_style_padding_right());
	assert_eq!(StyleUnit::UndefinedValue, root.get_style_padding_bottom());
	assert_eq!(StyleUnit::UndefinedValue, root.get_style_padding_start());
	assert_eq!(StyleUnit::UndefinedValue, root.get_style_padding_end());

//	assert!(root.get_style_border_left().is_nan());
//	assert!(root.get_style_border_top().is_nan());
//	assert!(root.get_style_border_right().is_nan());
//	assert!(root.get_style_border_bottom().is_nan());
//	assert!(root.get_style_border_start().is_nan());
//	assert!(root.get_style_border_end().is_nan());

	assert_eq!(StyleUnit::Auto, root.get_style_width());
	assert_eq!(StyleUnit::Auto, root.get_style_height());
	assert_eq!(StyleUnit::UndefinedValue, root.get_style_min_width());
	assert_eq!(StyleUnit::UndefinedValue, root.get_style_min_height());
	assert_eq!(StyleUnit::UndefinedValue, root.get_style_max_width());
	assert_eq!(StyleUnit::UndefinedValue, root.get_style_max_height());

	assert_eq!(0.0, root.get_layout_left());
	assert_eq!(0.0, root.get_layout_right());
	assert_eq!(0.0, root.get_layout_top());
	assert_eq!(0.0, root.get_layout_bottom());

	assert_eq!(0.0, root.get_layout_margin_left());
	assert_eq!(0.0, root.get_layout_margin_right());
	assert_eq!(0.0, root.get_layout_margin_top());
	assert_eq!(0.0, root.get_layout_margin_bottom());

	assert_eq!(0.0, root.get_layout_padding_left());
	assert_eq!(0.0, root.get_layout_padding_right());
	assert_eq!(0.0, root.get_layout_padding_top());
	assert_eq!(0.0, root.get_layout_padding_bottom());

	assert_eq!(0.0, root.get_layout_border_left());
	assert_eq!(0.0, root.get_layout_border_right());
	assert_eq!(0.0, root.get_layout_border_top());
	assert_eq!(0.0, root.get_layout_border_bottom());

//	assert!(root.get_layout_width().is_nan());
//	assert!(root.get_layout_height().is_nan());
	assert_eq!(Direction::Inherit, root.get_layout_direction());
}

#[test]
fn test_assert_webdefault_values() {
	// TODO - unimplemented for now
	assert!(true);
}

#[test]
fn test_assert_webdefault_values_reset() {
	// TODO - unimplemented for now
	assert!(true);
}
