extern crate yoga;

use yoga::{ContextNode, Direction, MeasureMode, NodeRef, Size, Undefined};

#[test]
fn test_context_1() {
	let mut root = ContextNode::new();
	let ref mut context = "test".to_string();
	root.set_context(context);

	let retrieved_context = root.get_own_context();

	assert_eq!(context, retrieved_context);
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
		let context: &String = ContextNode::get_context(&node_ref);

		Size {
			width: context.len() as f32,
			height: if context == "test" { 1.0 } else { 0.0 },
		}
	}

	let mut root = ContextNode::new();
	let ref mut context = "test".to_string();
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

	struct CustomData<'a> {
		text: String,
		font: &'a SimpleFont,
	}

	extern "C" fn measure(
		node_ref: NodeRef,
		_: f32,
		_: MeasureMode,
		_: f32,
		_: MeasureMode,
	) -> Size {
		let context: &CustomData = ContextNode::get_context(&node_ref);

		Size {
			width: context.text.len() as f32 * context.font.letter_width,
			height: context.font.letter_height,
		}
	}

	let shared_font = SimpleFont {
		letter_width: 2.0,
		letter_height: 10.0,
	};

	let mut data = CustomData {
		text: "hello world".to_string(),
		font: &shared_font,
	};

	let mut root = ContextNode::new();
	root.set_context(&mut data);
	root.set_measure_func(Some(measure));

	root.calculate_layout(Undefined, Undefined, Direction::LTR);

	let root_layout = root.get_layout();

	assert_eq!(0.0, root_layout.left);
	assert_eq!(0.0, root_layout.top);
	assert_eq!(22.0, root_layout.width);
	assert_eq!(10.0, root_layout.height);
}
