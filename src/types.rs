pub use crate::ffi_types::{
    align::*, config_ref::*, dimension::*, direction::*, display::*, edge::*, flex_direction::*,
    gutter::*, justify::*, log_level::*, measure_mode::*, node_ref::*, node_type::*, overflow::*,
    position_type::*, print_options::*, size::*, style_unit::*, undefined::*, wrap::*,
};
use ordered_float::OrderedFloat;
use std::{any::Any, ops::Deref, os::raw::c_void};

pub type BaselineFunc = Option<extern "C" fn(NodeRef, f32, f32) -> f32>;
pub type MeasureFunc = Option<extern "C" fn(NodeRef, f32, MeasureMode, f32, MeasureMode) -> Size>;

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
pub struct Context(Box<dyn Any>);

impl Context {
    pub fn new<T: Any>(value: T) -> Self {
        Context(Box::new(value))
    }

    pub(crate) fn into_raw(self) -> *mut c_void {
        // https://users.rust-lang.org/t/ffi-boxes-and-returning-references-to-the-boxed-data
        Box::into_raw(Box::new(self.0)) as *mut c_void
    }

    pub(crate) fn get_inner_ref<'a>(raw: *mut c_void) -> Option<&'a Box<dyn Any>> {
        let ptr = raw as *const Box<dyn Any>;
        unsafe { ptr.as_ref() }
    }

    pub(crate) fn get_inner_mut<'a>(raw: *mut c_void) -> Option<&'a mut Box<dyn Any>> {
        let ptr = raw as *mut Box<dyn Any>;
        unsafe { ptr.as_mut() }
    }

    pub(crate) fn drop_raw(raw: *mut c_void) {
        let ptr = raw as *mut Box<dyn Any>;
        if !ptr.is_null() {
            unsafe {
                let _ = Box::from_raw(ptr);
            }
        }
    }
}

impl Deref for Context {
    type Target = Box<dyn Any>;

    fn deref(&self) -> &Box<dyn Any> {
        &self.0
    }
}

#[macro_export]
macro_rules! unit {
    ($val:tt pt) => {
        $val.point()
    };
    ($val:tt %) => {
        $val.percent()
    };
    ($val:expr) => {
        $val
    };
}

#[macro_export]
macro_rules! flex_style {
	// Manually match on styles which require an OrderedFloat
	// This way the styles like
	//     Flex(1.0)
	// will be converted to:
	//     Flex(1.0.into())
	(AspectRatio($val:expr)) => (
		AspectRatio($val.into())
	);
	(BorderBottom($val:expr)) => (
		BorderBottom($val.into())
	);
	(BorderEnd($val:expr)) => (
		BorderEnd($val.into())
	);
	(BorderLeft($val:expr)) => (
		BorderLeft($val.into())
	);
	(BorderRight($val:expr)) => (
		BorderRight($val.into())
	);
	(BorderStart($val:expr)) => (
		BorderStart($val.into())
	);
	(BorderTop($val:expr)) => (
		BorderTop($val.into())
	);
	(Border($val:expr)) => (
		Border($val.into())
	);
	(Flex($val:expr)) => (
		Flex($val.into())
	);
	(FlexGrow($val:expr)) => (
		FlexGrow($val.into())
	);
	(FlexShrink($val:expr)) => (
		FlexShrink($val.into())
	);
	($s:ident($($unit:tt)*)) => (
		$s(unit!($($unit)*))
	);
}

#[macro_export]
macro_rules! style {
	( $x:expr, $($s:tt($($unit:tt)*)),* ) => {
		$x.apply_styles(&vec!(
			$(
				flex_style!($s(unit!($($unit)*))),
			)*
		))
	};
}

#[macro_export]
macro_rules! make_styles {
	( $($s:tt($($unit:tt)*)),* ) => {
		vec!(
			$(
				flex_style!($s(unit!($($unit)*))),
			)*
		)
	};
}
