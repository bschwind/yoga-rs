extern crate yoga;

use std::cell::RefCell;
use std::rc::Rc;
use yoga::{Context, Direction, Node, YGInternalNodeRef, Size, Undefined, YGInternalMeasureMode,
           YGInternalSize};

#[test]
fn test_context_0() {
	let root = Node::new();
	assert!(root.get_own_context().is_none());
	assert!(root.get_own_context_mut().is_none());
}

#[test]
fn test_context_1() {
	let mut root = Node::new();
	root.set_context(Some(Context::new("test".to_string())));

	let context = root.get_own_context().unwrap();
	assert!(context.downcast_ref::<bool>().is_none());
	assert!(context.downcast_ref::<&str>().is_none());
	assert_eq!(context.downcast_ref::<String>().unwrap(), "test");
}

#[test]
fn test_context_2_safe_check() {
	#[derive(Debug, PartialEq)]
	struct Bogus {}

	extern "C" fn measure(
		node_ref: YGInternalNodeRef,
		_: f32,
		_: YGInternalMeasureMode,
		_: f32,
		_: YGInternalMeasureMode,
	) -> YGInternalSize {
		let context = Node::get_context(&node_ref)
			.unwrap()
			.downcast_ref::<Bogus>();

		assert_eq!(context, None);

		Size {
			width: 1.0,
			height: 1.0,
		}.into()
	}

	let mut root = Node::new();
	let context = Context::new("test".to_string());
	root.set_context(Some(context));
	root.set_measure_func(Some(measure));

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(1.0, root_layout.width());
	assert_eq!(1.0, root_layout.height());
}

#[test]
fn test_context_2() {
	extern "C" fn measure(
		node_ref: YGInternalNodeRef,
		_: f32,
		_: YGInternalMeasureMode,
		_: f32,
		_: YGInternalMeasureMode,
	) -> YGInternalSize {
		let context = Node::get_context(&node_ref)
			.unwrap()
			.downcast_ref::<String>()
			.unwrap();

		YGInternalSize {
			width: context.len() as f32,
			height: if context == "test" { 1.0 } else { 0.0 },
		}
	}

	let mut root = Node::new();
	let context = Context::new("test".to_string());
	root.set_context(Some(context));
	root.set_measure_func(Some(measure));

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(4.0, root_layout.width());
	assert_eq!(1.0, root_layout.height());
}

#[test]
fn test_context_3() {
	struct LocalSimpleFont {
		letter_width: f32,
		letter_height: f32,
	}

	struct LocalCustomData {
		text: String,
		font: Rc<RefCell<LocalSimpleFont>>,
	}

	extern "C" fn measure(
		node_ref: YGInternalNodeRef,
		_: f32,
		_: YGInternalMeasureMode,
		_: f32,
		_: YGInternalMeasureMode,
	) -> YGInternalSize {
		let context = Node::get_context(&node_ref)
			.unwrap()
			.downcast_ref::<LocalCustomData>()
			.unwrap();

		let text = &context.text;
		let font = context.font.borrow();

		Size {
			width: text.len() as f32 * font.letter_width,
			height: font.letter_height,
		}.into()
	}

	let shared_font = Rc::new(RefCell::new(LocalSimpleFont {
		letter_width: 2.0,
		letter_height: 10.0,
	}));

	let data = Context::new(LocalCustomData {
		text: "hello world".to_string(),
		font: shared_font,
	});

	let mut root = Node::new();
	root.set_context(Some(data));
	root.set_measure_func(Some(measure));

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(22.0, root_layout.width());
	assert_eq!(10.0, root_layout.height());
}

struct ExternalSimpleFont {
	letter_width: f32,
	letter_height: f32,
}

struct ExternalCustomData {
	text: String,
	font: Rc<RefCell<ExternalSimpleFont>>,
}

#[test]
fn test_context_4() {
	let shared_font = Rc::new(RefCell::new(ExternalSimpleFont {
		letter_width: 2.0,
		letter_height: 10.0,
	}));

	let data = Context::new(ExternalCustomData {
		text: "hello world".to_string(),
		font: shared_font,
	});

	let mut root = Node::new();
	root.set_context(Some(data));
	root.set_measure_func(Some(external_measure));

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();

	assert_eq!(0.0, root_layout.left());
	assert_eq!(0.0, root_layout.top());
	assert_eq!(22.0, root_layout.width());
	assert_eq!(10.0, root_layout.height());
}

extern "C" fn external_measure(
	node_ref: YGInternalNodeRef,
	_: f32,
	_: YGInternalMeasureMode,
	_: f32,
	_: YGInternalMeasureMode,
) -> YGInternalSize {
	let context = Node::get_context(&node_ref)
		.unwrap()
		.downcast_ref::<ExternalCustomData>()
		.unwrap();

	let text = &context.text;
	let font = context.font.borrow();

	YGInternalSize {
		width: text.len() as f32 * font.letter_width,
		height: font.letter_height,
	}
}
