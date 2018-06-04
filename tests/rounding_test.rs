extern crate yoga;

use yoga::{Direction, Edge, FlexDirection, Node, StyleUnit, Undefined};

#[test]
fn test_rounding_flex_basis_flex_grow_row_width_of_100() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_flex_grow(1.0);
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_flex_grow(1.0);
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(33, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	assert_eq!(33, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(34, root_child1.get_layout_width() as i32);
	assert_eq!(100, root_child1.get_layout_height() as i32);

	assert_eq!(67, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(33, root_child2.get_layout_width() as i32);
	assert_eq!(100, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(67, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(33, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	assert_eq!(33, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(34, root_child1.get_layout_width() as i32);
	assert_eq!(100, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(33, root_child2.get_layout_width() as i32);
	assert_eq!(100, root_child2.get_layout_height() as i32);
}

#[test]
fn test_rounding_flex_basis_flex_grow_row_prime_number_width() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point(113.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_flex_grow(1.0);
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_flex_grow(1.0);
	root.insert_child(&mut root_child2, 2);

	let mut root_child3 = Node::new();
	root_child3.set_flex_grow(1.0);
	root.insert_child(&mut root_child3, 3);

	let mut root_child4 = Node::new();
	root_child4.set_flex_grow(1.0);
	root.insert_child(&mut root_child4, 4);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(113, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(23, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	assert_eq!(23, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(22, root_child1.get_layout_width() as i32);
	assert_eq!(100, root_child1.get_layout_height() as i32);

	assert_eq!(45, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(23, root_child2.get_layout_width() as i32);
	assert_eq!(100, root_child2.get_layout_height() as i32);

	assert_eq!(68, root_child3.get_layout_left() as i32);
	assert_eq!(0, root_child3.get_layout_top() as i32);
	assert_eq!(22, root_child3.get_layout_width() as i32);
	assert_eq!(100, root_child3.get_layout_height() as i32);

	assert_eq!(90, root_child4.get_layout_left() as i32);
	assert_eq!(0, root_child4.get_layout_top() as i32);
	assert_eq!(23, root_child4.get_layout_width() as i32);
	assert_eq!(100, root_child4.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(113, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(90, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(23, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	assert_eq!(68, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(22, root_child1.get_layout_width() as i32);
	assert_eq!(100, root_child1.get_layout_height() as i32);

	assert_eq!(45, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(23, root_child2.get_layout_width() as i32);
	assert_eq!(100, root_child2.get_layout_height() as i32);

	assert_eq!(23, root_child3.get_layout_left() as i32);
	assert_eq!(0, root_child3.get_layout_top() as i32);
	assert_eq!(22, root_child3.get_layout_width() as i32);
	assert_eq!(100, root_child3.get_layout_height() as i32);

	assert_eq!(0, root_child4.get_layout_left() as i32);
	assert_eq!(0, root_child4.get_layout_top() as i32);
	assert_eq!(23, root_child4.get_layout_width() as i32);
	assert_eq!(100, root_child4.get_layout_height() as i32);
}

#[test]
fn test_rounding_flex_basis_flex_shrink_row() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point(101.0.into()));
	root.set_height(StyleUnit::Point(100.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_shrink(1.0);
	root_child0.set_flex_basis(StyleUnit::Point(100.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_flex_basis(StyleUnit::Point(25.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_flex_basis(StyleUnit::Point(25.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(101, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(51, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	assert_eq!(51, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(25, root_child1.get_layout_width() as i32);
	assert_eq!(100, root_child1.get_layout_height() as i32);

	assert_eq!(76, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(25, root_child2.get_layout_width() as i32);
	assert_eq!(100, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(101, root.get_layout_width() as i32);
	assert_eq!(100, root.get_layout_height() as i32);

	assert_eq!(50, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(51, root_child0.get_layout_width() as i32);
	assert_eq!(100, root_child0.get_layout_height() as i32);

	assert_eq!(25, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(25, root_child1.get_layout_width() as i32);
	assert_eq!(100, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(25, root_child2.get_layout_width() as i32);
	assert_eq!(100, root_child2.get_layout_height() as i32);
}

#[test]
fn test_rounding_flex_basis_overrides_main_size() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(113.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_flex_basis(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(20.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_flex_grow(1.0);
	root_child1.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_flex_grow(1.0);
	root_child2.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(113, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(64, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(64, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(25, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(89, root_child2.get_layout_top() as i32);
	assert_eq!(100, root_child2.get_layout_width() as i32);
	assert_eq!(24, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(113, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(64, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(64, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(25, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(89, root_child2.get_layout_top() as i32);
	assert_eq!(100, root_child2.get_layout_width() as i32);
	assert_eq!(24, root_child2.get_layout_height() as i32);
}

#[test]
fn test_rounding_total_fractial() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(87.4.into()));
	root.set_height(StyleUnit::Point(113.4.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(0.7);
	root_child0.set_flex_basis(StyleUnit::Point(50.3.into()));
	root_child0.set_height(StyleUnit::Point(20.3.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_flex_grow(1.6);
	root_child1.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_flex_grow(1.1);
	root_child2.set_height(StyleUnit::Point(10.7.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(87, root.get_layout_width() as i32);
	assert_eq!(113, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(87, root_child0.get_layout_width() as i32);
	assert_eq!(59, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(59, root_child1.get_layout_top() as i32);
	assert_eq!(87, root_child1.get_layout_width() as i32);
	assert_eq!(30, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(89, root_child2.get_layout_top() as i32);
	assert_eq!(87, root_child2.get_layout_width() as i32);
	assert_eq!(24, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(87, root.get_layout_width() as i32);
	assert_eq!(113, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(87, root_child0.get_layout_width() as i32);
	assert_eq!(59, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(59, root_child1.get_layout_top() as i32);
	assert_eq!(87, root_child1.get_layout_width() as i32);
	assert_eq!(30, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(89, root_child2.get_layout_top() as i32);
	assert_eq!(87, root_child2.get_layout_width() as i32);
	assert_eq!(24, root_child2.get_layout_height() as i32);
}

#[test]
fn test_rounding_total_fractial_nested() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(87.4.into()));
	root.set_height(StyleUnit::Point(113.4.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(0.7);
	root_child0.set_flex_basis(StyleUnit::Point(50.3.into()));
	root_child0.set_height(StyleUnit::Point(20.3.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child0_child0 = Node::new();
	root_child0_child0.set_flex_grow(1.0);
	root_child0_child0.set_flex_basis(StyleUnit::Point(0.3.into()));
	root_child0_child0.set_position(Edge::Bottom, StyleUnit::Point(13.3.into()));
	root_child0_child0.set_height(StyleUnit::Point(9.9.into()));
	root_child0.insert_child(&mut root_child0_child0, 0);

	let mut root_child0_child1 = Node::new();
	root_child0_child1.set_flex_grow(4.0);
	root_child0_child1.set_flex_basis(StyleUnit::Point(0.3.into()));
	root_child0_child1.set_position(Edge::Top, StyleUnit::Point(13.3.into()));
	root_child0_child1.set_height(StyleUnit::Point(1.1.into()));
	root_child0.insert_child(&mut root_child0_child1, 1);

	let mut root_child1 = Node::new();
	root_child1.set_flex_grow(1.6);
	root_child1.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_flex_grow(1.1);
	root_child2.set_height(StyleUnit::Point(10.7.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(87, root.get_layout_width() as i32);
	assert_eq!(113, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(87, root_child0.get_layout_width() as i32);
	assert_eq!(59, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0.get_layout_left() as i32);
	assert_eq!(-13, root_child0_child0.get_layout_top() as i32);
	assert_eq!(87, root_child0_child0.get_layout_width() as i32);
	assert_eq!(12, root_child0_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child1.get_layout_left() as i32);
	assert_eq!(25, root_child0_child1.get_layout_top() as i32);
	assert_eq!(87, root_child0_child1.get_layout_width() as i32);
	assert_eq!(47, root_child0_child1.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(59, root_child1.get_layout_top() as i32);
	assert_eq!(87, root_child1.get_layout_width() as i32);
	assert_eq!(30, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(89, root_child2.get_layout_top() as i32);
	assert_eq!(87, root_child2.get_layout_width() as i32);
	assert_eq!(24, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(87, root.get_layout_width() as i32);
	assert_eq!(113, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(87, root_child0.get_layout_width() as i32);
	assert_eq!(59, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child0.get_layout_left() as i32);
	assert_eq!(-13, root_child0_child0.get_layout_top() as i32);
	assert_eq!(87, root_child0_child0.get_layout_width() as i32);
	assert_eq!(12, root_child0_child0.get_layout_height() as i32);

	assert_eq!(0, root_child0_child1.get_layout_left() as i32);
	assert_eq!(25, root_child0_child1.get_layout_top() as i32);
	assert_eq!(87, root_child0_child1.get_layout_width() as i32);
	assert_eq!(47, root_child0_child1.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(59, root_child1.get_layout_top() as i32);
	assert_eq!(87, root_child1.get_layout_width() as i32);
	assert_eq!(30, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(89, root_child2.get_layout_top() as i32);
	assert_eq!(87, root_child2.get_layout_width() as i32);
	assert_eq!(24, root_child2.get_layout_height() as i32);
}

#[test]
fn test_rounding_fractial_input_1() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(113.4.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_flex_basis(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(20.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_flex_grow(1.0);
	root_child1.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_flex_grow(1.0);
	root_child2.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(113, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(64, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(64, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(25, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(89, root_child2.get_layout_top() as i32);
	assert_eq!(100, root_child2.get_layout_width() as i32);
	assert_eq!(24, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(113, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(64, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(64, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(25, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(89, root_child2.get_layout_top() as i32);
	assert_eq!(100, root_child2.get_layout_width() as i32);
	assert_eq!(24, root_child2.get_layout_height() as i32);
}

#[test]
fn test_rounding_fractial_input_2() {
	let mut root = Node::new();
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(113.6.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_flex_basis(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(20.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_flex_grow(1.0);
	root_child1.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_flex_grow(1.0);
	root_child2.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(114, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(65, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(65, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(24, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(89, root_child2.get_layout_top() as i32);
	assert_eq!(100, root_child2.get_layout_width() as i32);
	assert_eq!(25, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(114, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(65, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(65, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(24, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(89, root_child2.get_layout_top() as i32);
	assert_eq!(100, root_child2.get_layout_width() as i32);
	assert_eq!(25, root_child2.get_layout_height() as i32);
}

#[test]
fn test_rounding_fractial_input_3() {
	let mut root = Node::new();
	root.set_position(Edge::Top, StyleUnit::Point(0.3.into()));
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(113.4.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_flex_basis(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(20.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_flex_grow(1.0);
	root_child1.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_flex_grow(1.0);
	root_child2.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(114, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(65, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(64, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(24, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(89, root_child2.get_layout_top() as i32);
	assert_eq!(100, root_child2.get_layout_width() as i32);
	assert_eq!(25, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(114, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(65, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(64, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(24, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(89, root_child2.get_layout_top() as i32);
	assert_eq!(100, root_child2.get_layout_width() as i32);
	assert_eq!(25, root_child2.get_layout_height() as i32);
}

#[test]
fn test_rounding_fractial_input_4() {
	let mut root = Node::new();
	root.set_position(Edge::Top, StyleUnit::Point(0.7.into()));
	root.set_width(StyleUnit::Point(100.0.into()));
	root.set_height(StyleUnit::Point(113.4.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_flex_basis(StyleUnit::Point(50.0.into()));
	root_child0.set_height(StyleUnit::Point(20.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_flex_grow(1.0);
	root_child1.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child2 = Node::new();
	root_child2.set_flex_grow(1.0);
	root_child2.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(1, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(113, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(64, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(64, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(25, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(89, root_child2.get_layout_top() as i32);
	assert_eq!(100, root_child2.get_layout_width() as i32);
	assert_eq!(24, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(1, root.get_layout_top() as i32);
	assert_eq!(100, root.get_layout_width() as i32);
	assert_eq!(113, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(100, root_child0.get_layout_width() as i32);
	assert_eq!(64, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(64, root_child1.get_layout_top() as i32);
	assert_eq!(100, root_child1.get_layout_width() as i32);
	assert_eq!(25, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(89, root_child2.get_layout_top() as i32);
	assert_eq!(100, root_child2.get_layout_width() as i32);
	assert_eq!(24, root_child2.get_layout_height() as i32);
}

#[test]
fn test_rounding_inner_node_controversy_horizontal() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point(320.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_flex_grow(1.0);
	root_child1.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child1_child0 = Node::new();
	root_child1_child0.set_flex_grow(1.0);
	root_child1_child0.set_height(StyleUnit::Point(10.0.into()));
	root_child1.insert_child(&mut root_child1_child0, 0);

	let mut root_child2 = Node::new();
	root_child2.set_flex_grow(1.0);
	root_child2.set_height(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(320, root.get_layout_width() as i32);
//	assert_eq!(10, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(107, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(107, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(106, root_child1.get_layout_width() as i32);
	assert_eq!(10, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child1_child0.get_layout_left() as i32);
	assert_eq!(0, root_child1_child0.get_layout_top() as i32);
	assert_eq!(106, root_child1_child0.get_layout_width() as i32);
	assert_eq!(10, root_child1_child0.get_layout_height() as i32);

	assert_eq!(213, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(107, root_child2.get_layout_width() as i32);
	assert_eq!(10, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(320, root.get_layout_width() as i32);
//	assert_eq!(10, root.get_layout_height() as i32);

	assert_eq!(213, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(107, root_child0.get_layout_width() as i32);
	assert_eq!(10, root_child0.get_layout_height() as i32);

	assert_eq!(107, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(106, root_child1.get_layout_width() as i32);
	assert_eq!(10, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child1_child0.get_layout_left() as i32);
	assert_eq!(0, root_child1_child0.get_layout_top() as i32);
	assert_eq!(106, root_child1_child0.get_layout_width() as i32);
	assert_eq!(10, root_child1_child0.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(107, root_child2.get_layout_width() as i32);
	assert_eq!(10, root_child2.get_layout_height() as i32);
}

#[test]
fn test_rounding_inner_node_controversy_vertical() {
	let mut root = Node::new();
	root.set_height(StyleUnit::Point(320.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_flex_grow(1.0);
	root_child1.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child1_child0 = Node::new();
	root_child1_child0.set_flex_grow(1.0);
	root_child1_child0.set_width(StyleUnit::Point(10.0.into()));
	root_child1.insert_child(&mut root_child1_child0, 0);

	let mut root_child2 = Node::new();
	root_child2.set_flex_grow(1.0);
	root_child2.set_width(StyleUnit::Point(10.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
//	assert_eq!(10, root.get_layout_width() as i32);
	assert_eq!(320, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(107, root_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(107, root_child1.get_layout_top() as i32);
	assert_eq!(10, root_child1.get_layout_width() as i32);
	assert_eq!(106, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child1_child0.get_layout_left() as i32);
	assert_eq!(0, root_child1_child0.get_layout_top() as i32);
	assert_eq!(10, root_child1_child0.get_layout_width() as i32);
	assert_eq!(106, root_child1_child0.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(213, root_child2.get_layout_top() as i32);
	assert_eq!(10, root_child2.get_layout_width() as i32);
	assert_eq!(107, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
//	assert_eq!(10, root.get_layout_width() as i32);
	assert_eq!(320, root.get_layout_height() as i32);

//	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(10, root_child0.get_layout_width() as i32);
	assert_eq!(107, root_child0.get_layout_height() as i32);

//	assert_eq!(0, root_child1.get_layout_left() as i32);
	assert_eq!(107, root_child1.get_layout_top() as i32);
	assert_eq!(10, root_child1.get_layout_width() as i32);
	assert_eq!(106, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child1_child0.get_layout_left() as i32);
	assert_eq!(0, root_child1_child0.get_layout_top() as i32);
	assert_eq!(10, root_child1_child0.get_layout_width() as i32);
	assert_eq!(106, root_child1_child0.get_layout_height() as i32);

//	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(213, root_child2.get_layout_top() as i32);
	assert_eq!(10, root_child2.get_layout_width() as i32);
	assert_eq!(107, root_child2.get_layout_height() as i32);
}

#[test]
fn test_rounding_inner_node_controversy_combined() {
	let mut root = Node::new();
	root.set_flex_direction(FlexDirection::Row);
	root.set_width(StyleUnit::Point(640.0.into()));
	root.set_height(StyleUnit::Point(320.0.into()));

	let mut root_child0 = Node::new();
	root_child0.set_flex_grow(1.0);
	root_child0.set_height(StyleUnit::Percent(100.0.into()));
	root.insert_child(&mut root_child0, 0);

	let mut root_child1 = Node::new();
	root_child1.set_flex_grow(1.0);
	root_child1.set_height(StyleUnit::Percent(100.0.into()));
	root.insert_child(&mut root_child1, 1);

	let mut root_child1_child0 = Node::new();
	root_child1_child0.set_flex_grow(1.0);
	root_child1_child0.set_width(StyleUnit::Percent(100.0.into()));
	root_child1.insert_child(&mut root_child1_child0, 0);

	let mut root_child1_child1 = Node::new();
	root_child1_child1.set_flex_grow(1.0);
	root_child1_child1.set_width(StyleUnit::Percent(100.0.into()));
	root_child1.insert_child(&mut root_child1_child1, 1);

	let mut root_child1_child1_child0 = Node::new();
	root_child1_child1_child0.set_flex_grow(1.0);
	root_child1_child1_child0.set_width(StyleUnit::Percent(100.0.into()));
	root_child1_child1.insert_child(&mut root_child1_child1_child0, 0);

	let mut root_child1_child2 = Node::new();
	root_child1_child2.set_flex_grow(1.0);
	root_child1_child2.set_width(StyleUnit::Percent(100.0.into()));
	root_child1.insert_child(&mut root_child1_child2, 2);

	let mut root_child2 = Node::new();
	root_child2.set_flex_grow(1.0);
	root_child2.set_height(StyleUnit::Percent(100.0.into()));
	root.insert_child(&mut root_child2, 2);
	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(640, root.get_layout_width() as i32);
	assert_eq!(320, root.get_layout_height() as i32);

	assert_eq!(0, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(213, root_child0.get_layout_width() as i32);
	assert_eq!(320, root_child0.get_layout_height() as i32);

	assert_eq!(213, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(214, root_child1.get_layout_width() as i32);
	assert_eq!(320, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child1_child0.get_layout_left() as i32);
	assert_eq!(0, root_child1_child0.get_layout_top() as i32);
	assert_eq!(214, root_child1_child0.get_layout_width() as i32);
	assert_eq!(107, root_child1_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1_child1.get_layout_left() as i32);
	assert_eq!(107, root_child1_child1.get_layout_top() as i32);
	assert_eq!(214, root_child1_child1.get_layout_width() as i32);
	assert_eq!(106, root_child1_child1.get_layout_height() as i32);

	assert_eq!(0, root_child1_child1_child0.get_layout_left() as i32);
	assert_eq!(0, root_child1_child1_child0.get_layout_top() as i32);
	assert_eq!(214, root_child1_child1_child0.get_layout_width() as i32);
	assert_eq!(106, root_child1_child1_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1_child2.get_layout_left() as i32);
	assert_eq!(213, root_child1_child2.get_layout_top() as i32);
	assert_eq!(214, root_child1_child2.get_layout_width() as i32);
	assert_eq!(107, root_child1_child2.get_layout_height() as i32);

	assert_eq!(427, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(213, root_child2.get_layout_width() as i32);
	assert_eq!(320, root_child2.get_layout_height() as i32);

	root.calculate_layout(Undefined, Undefined, Direction::RTL);

	assert_eq!(0, root.get_layout_left() as i32);
	assert_eq!(0, root.get_layout_top() as i32);
	assert_eq!(640, root.get_layout_width() as i32);
	assert_eq!(320, root.get_layout_height() as i32);

	assert_eq!(427, root_child0.get_layout_left() as i32);
	assert_eq!(0, root_child0.get_layout_top() as i32);
	assert_eq!(213, root_child0.get_layout_width() as i32);
	assert_eq!(320, root_child0.get_layout_height() as i32);

	assert_eq!(213, root_child1.get_layout_left() as i32);
	assert_eq!(0, root_child1.get_layout_top() as i32);
	assert_eq!(214, root_child1.get_layout_width() as i32);
	assert_eq!(320, root_child1.get_layout_height() as i32);

	assert_eq!(0, root_child1_child0.get_layout_left() as i32);
	assert_eq!(0, root_child1_child0.get_layout_top() as i32);
	assert_eq!(214, root_child1_child0.get_layout_width() as i32);
	assert_eq!(107, root_child1_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1_child1.get_layout_left() as i32);
	assert_eq!(107, root_child1_child1.get_layout_top() as i32);
	assert_eq!(214, root_child1_child1.get_layout_width() as i32);
	assert_eq!(106, root_child1_child1.get_layout_height() as i32);

	assert_eq!(0, root_child1_child1_child0.get_layout_left() as i32);
	assert_eq!(0, root_child1_child1_child0.get_layout_top() as i32);
	assert_eq!(214, root_child1_child1_child0.get_layout_width() as i32);
	assert_eq!(106, root_child1_child1_child0.get_layout_height() as i32);

	assert_eq!(0, root_child1_child2.get_layout_left() as i32);
	assert_eq!(213, root_child1_child2.get_layout_top() as i32);
	assert_eq!(214, root_child1_child2.get_layout_width() as i32);
	assert_eq!(107, root_child1_child2.get_layout_height() as i32);

	assert_eq!(0, root_child2.get_layout_left() as i32);
	assert_eq!(0, root_child2.get_layout_top() as i32);
	assert_eq!(213, root_child2.get_layout_width() as i32);
	assert_eq!(320, root_child2.get_layout_height() as i32);
}
