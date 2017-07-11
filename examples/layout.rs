#[macro_use]
extern crate yoga;

use yoga::{Node, Point, Percent};
use yoga::FlexStyle::*;
use yoga::StyleUnit::{Auto, UndefinedValue};

fn main() {
	let mut node = Node::new();

	let mut child = Node::new();
	let mut other_child = Node::new();

	node.insert_child(&mut child, 0);
	node.insert_child(&mut other_child, 1);

	style!(node,
		Margin(10 pt),
		MarginLeft(Auto),
		PaddingHorizontal(4 pt),
		Left(16 %),
		Bottom(UndefinedValue)
	);

	let child_styles = make_styles!(
		Width(32 pt),
		Height(32 pt),
		Margin(Auto)
	);

	child.apply_styles(&child_styles);
	other_child.apply_styles(&child_styles);

	node.calculate_layout(512.0, 512.0, yoga::Direction::LTR);

	println!("Layout is {:#?}", node.get_layout());
}
