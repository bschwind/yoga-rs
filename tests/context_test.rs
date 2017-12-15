extern crate yoga;

use std::cell::RefCell;
use std::rc::Rc;
use yoga::{Context, Direction, MeasureMode, Node, NodeRef, Size, Undefined};

#[test]
fn test_context_1() {
	let mut root = Node::new();
	let ref mut context = Context::new("test".to_string());
	root.set_context(context);

	let retrieved_context = root.get_own_context();

	assert_eq!(
		context.downcast_ref::<String>().unwrap(),
		retrieved_context.downcast_ref::<String>().unwrap()
	);
}

#[test]
fn test_context_2_safe_check() {
	#[derive(Debug, PartialEq)]
	struct Bogus {}

	extern "C" fn measure(
		node_ref: NodeRef,
		_: f32,
		_: MeasureMode,
		_: f32,
		_: MeasureMode,
	) -> Size {
		let context = Node::get_context(&node_ref).downcast_ref::<Bogus>();

		assert_eq!(context, None);

		Size {
			width: 1.0,
			height: 1.0,
		}
	}

	let mut root = Node::new();
	let ref mut context = Context::new("test".to_string());
	root.set_context(context);
	root.set_measure_func(Some(measure));

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(1.0, root_layout.width);
	assert_eq!(1.0, root_layout.height);
}

#[test]
fn test_context_2() {
	extern "C" fn measure(
		node_ref: NodeRef,
		_: f32,
		_: MeasureMode,
		_: f32,
		_: MeasureMode,
	) -> Size {
		let context = Node::get_context(&node_ref)
			.downcast_ref::<String>()
			.unwrap();

		Size {
			width: context.len() as f32,
			height: if context == "test" { 1.0 } else { 0.0 },
		}
	}

	let mut root = Node::new();
	let ref mut context = Context::new("test".to_string());
	root.set_context(context);
	root.set_measure_func(Some(measure));

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(4.0, root_layout.width);
	assert_eq!(1.0, root_layout.height);
}

#[test]
fn test_context_3() {
	struct SimpleFont {
		letter_width: f32,
		letter_height: f32,
	}

	struct CustomData {
		text: String,
		font: Rc<RefCell<SimpleFont>>,
	}

	extern "C" fn measure(
		node_ref: NodeRef,
		_: f32,
		_: MeasureMode,
		_: f32,
		_: MeasureMode,
	) -> Size {
		let context = Node::get_context(&node_ref)
			.downcast_ref::<CustomData>()
			.unwrap();

		let text = &context.text;
		let font = context.font.borrow();

		Size {
			width: text.len() as f32 * font.letter_width,
			height: font.letter_height,
		}
	}

	let shared_font = Rc::new(RefCell::new(SimpleFont {
		letter_width: 2.0,
		letter_height: 10.0,
	}));

	let mut data = Context::new(CustomData {
		text: "hello world".to_string(),
		font: shared_font,
	});

	let mut root = Node::new();
	root.set_context(&mut data);
	root.set_measure_func(Some(measure));

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(22.0, root_layout.width);
	assert_eq!(10.0, root_layout.height);
}
