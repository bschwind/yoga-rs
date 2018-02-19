pub use ffi_types::align::*;
pub use ffi_types::dimension::*;
pub use ffi_types::direction::*;
pub use ffi_types::display::*;
pub use ffi_types::edge::*;
pub use ffi_types::flex_direction::*;
pub use ffi_types::justify::*;
pub use ffi_types::log_level::*;
pub use ffi_types::measure_mode::*;
pub use ffi_types::node_ref::*;
pub use ffi_types::node_type::*;
pub use ffi_types::overflow::*;
pub use ffi_types::position_type::*;
pub use ffi_types::print_options::*;
pub use ffi_types::size::*;
pub use ffi_types::style_unit::*;
pub use ffi_types::undefined::*;
pub use ffi_types::wrap::*;
use ordered_float::OrderedFloat;
use std::any::Any;
use std::ops::Deref;
use std::os::raw::c_void;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum FlexStyle {
	AlignContent(Align),
	AlignItems(Align),
	AlignSelf(Align),
	AspectRatio(OrderedFloat<f32>),
	BorderBottom(OrderedFloat<f32>),
	BorderEnd(OrderedFloat<f32>),
	BorderLeft(OrderedFloat<f32>),
	BorderRight(OrderedFloat<f32>),
	BorderStart(OrderedFloat<f32>),
	BorderTop(OrderedFloat<f32>),
	Border(OrderedFloat<f32>),
	Bottom(StyleUnit),
	Display(Display),
	End(StyleUnit),
	Flex(OrderedFloat<f32>),
	FlexBasis(StyleUnit),
	FlexDirection(FlexDirection),
	FlexGrow(OrderedFloat<f32>),
	FlexShrink(OrderedFloat<f32>),
	FlexWrap(Wrap),
	Height(StyleUnit),
	JustifyContent(Justify),
	Left(StyleUnit),
	Margin(StyleUnit),
	MarginBottom(StyleUnit),
	MarginEnd(StyleUnit),
	MarginHorizontal(StyleUnit),
	MarginLeft(StyleUnit),
	MarginRight(StyleUnit),
	MarginStart(StyleUnit),
	MarginTop(StyleUnit),
	MarginVertical(StyleUnit),
	MaxHeight(StyleUnit),
	MaxWidth(StyleUnit),
	MinHeight(StyleUnit),
	MinWidth(StyleUnit),
	Overflow(Overflow),
	Padding(StyleUnit),
	PaddingBottom(StyleUnit),
	PaddingEnd(StyleUnit),
	PaddingHorizontal(StyleUnit),
	PaddingLeft(StyleUnit),
	PaddingRight(StyleUnit),
	PaddingStart(StyleUnit),
	PaddingTop(StyleUnit),
	PaddingVertical(StyleUnit),
	Position(PositionType),
	Right(StyleUnit),
	Start(StyleUnit),
	Top(StyleUnit),
	Width(StyleUnit),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Layout {
	left: OrderedFloat<f32>,
	right: OrderedFloat<f32>,
	top: OrderedFloat<f32>,
	bottom: OrderedFloat<f32>,
	width: OrderedFloat<f32>,
	height: OrderedFloat<f32>,
}

impl Layout {
	pub fn new(left: f32, right: f32, top: f32, bottom: f32, width: f32, height: f32) -> Layout {
		Layout {
			left: left.into(),
			right: right.into(),
			top: top.into(),
			bottom: bottom.into(),
			width: width.into(),
			height: height.into(),
		}
	}

	pub fn left(&self) -> f32 {
		self.left.into_inner()
	}

	pub fn right(&self) -> f32 {
		self.right.into_inner()
	}

	pub fn top(&self) -> f32 {
		self.top.into_inner()
	}

	pub fn bottom(&self) -> f32 {
		self.bottom.into_inner()
	}

	pub fn width(&self) -> f32 {
		self.width.into_inner()
	}

	pub fn height(&self) -> f32 {
		self.height.into_inner()
	}
}

#[derive(Debug)]
pub struct Context(Box<Any>);

impl Context {
	pub fn new<T: Any>(value: T) -> Self {
		Context(Box::new(value))
	}

	pub(crate) fn into_raw(self) -> *mut c_void {
		// https://users.rust-lang.org/t/ffi-boxes-and-returning-references-to-the-boxed-data
		Box::into_raw(Box::new(self.0)) as *mut c_void
	}

	pub(crate) fn get_inner_ref<'a>(raw: *mut c_void) -> Option<&'a Box<Any>> {
		let ptr = raw as *const Box<Any>;
		unsafe { ptr.as_ref() }
	}

	pub(crate) fn get_inner_mut<'a>(raw: *mut c_void) -> Option<&'a mut Box<Any>> {
		let ptr = raw as *mut Box<Any>;
		unsafe { ptr.as_mut() }
	}

	pub(crate) fn drop_raw(raw: *mut c_void) {
		let ptr = raw as *mut Box<Any>;
		if !ptr.is_null() {
			unsafe {
				Box::from_raw(ptr);
			}
		}
	}
}

impl Deref for Context {
	type Target = Box<Any>;
	fn deref(&self) -> &Box<Any> {
		&self.0
	}
}
