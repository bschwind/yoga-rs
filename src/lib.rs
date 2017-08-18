#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate ordered_float;

// API created by bindgen
mod internal {
	#![allow(dead_code)]
	include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::convert::From;
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum FlexStyle {
	AlignContent(Align),
	AlignItems(Align),
	AlignSelf(Align),
	AspectRatio(OrderedFloat<f32>),
	BorderBottom(OrderedFloat<f32>),
	BorderLeft(OrderedFloat<f32>),
	BorderRight(OrderedFloat<f32>),
	BorderTop(OrderedFloat<f32>),
	Border(OrderedFloat<f32>),
	Bottom(StyleUnit),
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
	MarginHorizontal(StyleUnit),
	MarginLeft(StyleUnit),
	MarginRight(StyleUnit),
	MarginTop(StyleUnit),
	MarginVertical(StyleUnit),
	MaxHeight(StyleUnit),
	MaxWidth(StyleUnit),
	MinHeight(StyleUnit),
	MinWidth(StyleUnit),
	Overflow(Overflow),
	Padding(StyleUnit),
	PaddingBottom(StyleUnit),
	PaddingHorizontal(StyleUnit),
	PaddingLeft(StyleUnit),
	PaddingRight(StyleUnit),
	PaddingTop(StyleUnit),
	PaddingVertical(StyleUnit),
	Position(PositionType),
	Right(StyleUnit),
	Start(StyleUnit),
	Top(StyleUnit),
	Width(StyleUnit)
}

// Public re-exports of Yoga enums
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Align {
	Auto = 0,
	FlexStart = 1,
	Center = 2,
	FlexEnd = 3,
	Stretch = 4,
	Baseline = 5,
	SpaceBetween = 6,
	SpaceAround = 7
}

impl From<Align> for internal::YGAlign {
	fn from(a: Align) -> internal::YGAlign {
		match a {
			Align::Auto => internal::YGAlign::YGAlignAuto,
			Align::FlexStart => internal::YGAlign::YGAlignFlexStart,
			Align::Center => internal::YGAlign::YGAlignCenter,
			Align::FlexEnd => internal::YGAlign::YGAlignFlexEnd,
			Align::Stretch => internal::YGAlign::YGAlignStretch,
			Align::Baseline => internal::YGAlign::YGAlignBaseline,
			Align::SpaceBetween => internal::YGAlign::YGAlignSpaceBetween,
			Align::SpaceAround => internal::YGAlign::YGAlignSpaceAround
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Dimension {
	Width = 0,
	Height = 1
}

impl From<Dimension> for internal::YGDimension {
	fn from(d: Dimension) -> internal::YGDimension {
		match d {
			Dimension::Width => internal::YGDimension::YGDimensionWidth,
			Dimension::Height => internal::YGDimension::YGDimensionHeight
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
	Inherit = 0,
	LTR = 1,
	RTL = 2
}

impl From<Direction> for internal::YGDirection {
	fn from(d: Direction) -> internal::YGDirection {
		match d {
			Direction::Inherit => internal::YGDirection::YGDirectionInherit,
			Direction::LTR => internal::YGDirection::YGDirectionLTR,
			Direction::RTL => internal::YGDirection::YGDirectionRTL
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Display {
	Flex = 0,
	None = 1
}

impl From<Display> for internal::YGDisplay {
	fn from(d: Display) -> internal::YGDisplay {
		match d {
			Display::Flex => internal::YGDisplay::YGDisplayFlex,
			Display::None => internal::YGDisplay::YGDisplayNone
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Edge {
	Left = 0,
	Top = 1,
	Right = 2,
	Bottom = 3,
	Start = 4,
	End = 5,
	Horizontal = 6,
	Vertical = 7,
	All = 8
}

impl From<Edge> for internal::YGEdge {
	fn from(e: Edge) -> internal::YGEdge {
		match e {
			Edge::Left => internal::YGEdge::YGEdgeLeft,
			Edge::Top => internal::YGEdge::YGEdgeTop,
			Edge::Right => internal::YGEdge::YGEdgeRight,
			Edge::Bottom => internal::YGEdge::YGEdgeBottom,
			Edge::Start => internal::YGEdge::YGEdgeStart,
			Edge::End => internal::YGEdge::YGEdgeEnd,
			Edge::Horizontal => internal::YGEdge::YGEdgeHorizontal,
			Edge::Vertical => internal::YGEdge::YGEdgeVertical,
			Edge::All => internal::YGEdge::YGEdgeAll
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FlexDirection {
	Column = 0,
	ColumnReverse = 1,
	Row = 2,
	RowReverse = 3,
}

impl From<FlexDirection> for internal::YGFlexDirection {
	fn from(f: FlexDirection) -> internal::YGFlexDirection {
		match f {
			FlexDirection::Column => internal::YGFlexDirection::YGFlexDirectionColumn,
			FlexDirection::ColumnReverse => internal::YGFlexDirection::YGFlexDirectionColumnReverse,
			FlexDirection::Row => internal::YGFlexDirection::YGFlexDirectionRow,
			FlexDirection::RowReverse => internal::YGFlexDirection::YGFlexDirectionRowReverse
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Justify {
	FlexStart = 0,
	Center = 1,
	FlexEnd = 2,
	SpaceBetween = 3,
	SpaceAround = 4
}

impl From<Justify> for internal::YGJustify {
	fn from(j: Justify) -> internal::YGJustify {
		match j {
			Justify::FlexStart => internal::YGJustify::YGJustifyFlexStart,
			Justify::Center => internal::YGJustify::YGJustifyCenter,
			Justify::FlexEnd => internal::YGJustify::YGJustifyFlexEnd,
			Justify::SpaceBetween => internal::YGJustify::YGJustifySpaceBetween,
			Justify::SpaceAround => internal::YGJustify::YGJustifySpaceAround
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LogLevel {
	Error = 0,
	Warn = 1,
	Info = 2,
	Debug = 3,
	Verbose = 4,
	Fatal = 5
}

impl From<LogLevel> for internal::YGLogLevel {
	fn from(l: LogLevel) -> internal::YGLogLevel {
		match l {
			LogLevel::Error => internal::YGLogLevel::YGLogLevelError,
			LogLevel::Warn => internal::YGLogLevel::YGLogLevelWarn,
			LogLevel::Info => internal::YGLogLevel::YGLogLevelInfo,
			LogLevel::Debug => internal::YGLogLevel::YGLogLevelDebug,
			LogLevel::Verbose => internal::YGLogLevel::YGLogLevelVerbose,
			LogLevel::Fatal => internal::YGLogLevel::YGLogLevelFatal
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MeasureMode {
	Undefined = 0,
	Exactly = 1,
	AtMost = 2
}

impl From<MeasureMode> for internal::YGMeasureMode {
	fn from(m: MeasureMode) -> internal::YGMeasureMode {
		match m {
			MeasureMode::Undefined => internal::YGMeasureMode::YGMeasureModeUndefined,
			MeasureMode::Exactly => internal::YGMeasureMode::YGMeasureModeExactly,
			MeasureMode::AtMost => internal::YGMeasureMode::YGMeasureModeAtMost
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NodeType {
	Default = 0,
	Text = 1
}

impl From<NodeType> for internal::YGNodeType {
	fn from(n: NodeType) -> internal::YGNodeType {
		match n {
			NodeType::Default => internal::YGNodeType::YGNodeTypeDefault,
			NodeType::Text => internal::YGNodeType::YGNodeTypeText
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Overflow {
	Visible = 0,
	Hidden = 1,
	Scroll = 2
}

impl From<Overflow> for internal::YGOverflow {
	fn from(o: Overflow) -> internal::YGOverflow {
		match o {
			Overflow::Visible => internal::YGOverflow::YGOverflowVisible,
			Overflow::Hidden => internal::YGOverflow::YGOverflowHidden,
			Overflow::Scroll => internal::YGOverflow::YGOverflowScroll
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PositionType {
	Relative = 0,
	Absolute = 1
}

impl From<PositionType> for internal::YGPositionType {
	fn from(p: PositionType) -> internal::YGPositionType {
		match p {
			PositionType::Relative => internal::YGPositionType::YGPositionTypeRelative,
			PositionType::Absolute => internal::YGPositionType::YGPositionTypeAbsolute
		}
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PrintOptions {
	Layout = 1,
	Style = 2,
	Children = 4
}

impl From<PrintOptions> for internal::YGPrintOptions {
	fn from(p: PrintOptions) -> internal::YGPrintOptions {
		match p {
			PrintOptions::Layout => internal::YGPrintOptions::YGPrintOptionsLayout,
			PrintOptions::Style => internal::YGPrintOptions::YGPrintOptionsStyle,
			PrintOptions::Children => internal::YGPrintOptions::YGPrintOptionsChildren
		}
	}
}

#[repr(C)]
#[derive(Debug, Copy)]
pub struct Size {
	pub width: f32,
	pub height: f32
}

impl Clone for Size {
    fn clone(&self) -> Self { *self }
}

impl From<Size> for internal::YGSize {
	fn from(s: Size) -> internal::YGSize {
		internal::YGSize {
			width: s.width,
			height: s.height
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StyleUnit {
	UndefinedValue,
	Point(OrderedFloat<f32>),
	Percent(OrderedFloat<f32>),
	Auto
}

impl From<StyleUnit> for internal::YGUnit {
	fn from(s: StyleUnit) -> internal::YGUnit {
		match s {
			StyleUnit::UndefinedValue => internal::YGUnit::YGUnitUndefined,
			StyleUnit::Point(_) => internal::YGUnit::YGUnitPoint,
			StyleUnit::Percent(_) => internal::YGUnit::YGUnitPercent,
			StyleUnit::Auto => internal::YGUnit::YGUnitAuto
		}
	}
}

#[macro_export]
macro_rules! unit {
	( $val:tt pt) => (
		$val.point()
	);
	( $val:tt %) => {
		$val.percent()
	};
	( $val:expr) => {
		$val
	};
}

#[macro_export]
macro_rules! flex_style {
	// Manually match on styles which require an OrderedFloat
	// This way the styles like
	//     Flex(1.0)
	// will be converted to:
	//     Flex(OrderedFloat(1.0))
	(AspectRatio($val:expr)) => (
		AspectRatio(OrderedFloat($val))
	);
	(BorderBottom($val:expr)) => (
		BorderBottom(OrderedFloat($val))
	);
	(BorderLeft($val:expr)) => (
		BorderLeft(OrderedFloat($val))
	);
	(BorderRight($val:expr)) => (
		BorderRight(OrderedFloat($val))
	);
	(BorderTop($val:expr)) => (
		BorderTop(OrderedFloat($val))
	);
	(Border($val:expr)) => (
		Border(OrderedFloat($val))
	);
	(Flex($val:expr)) => (
		Flex(OrderedFloat($val))
	);
	(FlexGrow($val:expr)) => (
		FlexGrow(OrderedFloat($val))
	);
	(FlexShrink($val:expr)) => (
		FlexShrink(OrderedFloat($val))
	);
	($s:ident($($unit:tt)*)) => (
		$s(unit!($($unit)*))
	);
}

#[macro_export]
macro_rules! style {
	( $x:expr, $($s:ident($($unit:tt)*)),* ) => {
		$x.apply_styles(&vec!(
			$(
				flex_style!($s(unit!($($unit)*))),
			)*
		))
	};
}

#[macro_export]
macro_rules! make_styles {
	( $($s:ident($($unit:tt)*)),* ) => {
		vec!(
			$(
				$s(unit!($($unit)*)),
			)*
		)
	};
}

pub trait Percent {
	fn percent(self) -> StyleUnit;
}

impl Percent for f32 {
	fn percent(self) -> StyleUnit {
		StyleUnit::Percent(OrderedFloat(self))
	}
}

impl Percent for i32 {
	fn percent(self) -> StyleUnit {
		StyleUnit::Percent(OrderedFloat(self as f32))
	}
}

pub trait Point {
	fn point(self) -> StyleUnit;
}

impl Point for f32 {
	fn point(self) -> StyleUnit {
		StyleUnit::Point(OrderedFloat(self))
	}
}

impl Point for i32 {
	fn point(self) -> StyleUnit {
		StyleUnit::Point(OrderedFloat(self as f32))
	}
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Wrap {
	NoWrap = 0,
	Wrap = 1,
	WrapReverse = 2
}

impl From<Wrap> for internal::YGWrap {
	fn from(w: Wrap) -> internal::YGWrap {
		match w {
			Wrap::NoWrap => internal::YGWrap::YGWrapNoWrap,
			Wrap::Wrap => internal::YGWrap::YGWrapWrap,
			Wrap::WrapReverse => internal::YGWrap::YGWrapWrapReverse
		}
	}
}

pub use std::f32::NAN as Undefined;

// Custom Rust API

pub type NodeRef = internal::YGNodeRef;

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Node {
	inner_node: NodeRef,
	should_free: bool
}

#[derive(Debug, PartialEq)]
pub struct Layout {
	pub left: f32,
	pub right: f32,
	pub top: f32,
	pub bottom: f32,
	pub width: f32,
	pub height: f32
}

impl Node {
	pub fn new() -> Node {
		Node {
			inner_node: unsafe { internal::YGNodeNew() },
			should_free: true
		}
	}

	pub fn reset(&mut self) {
		unsafe {
			internal::YGNodeReset(self.inner_node);
		}
	}

	pub fn apply_styles<'a, I>(&mut self, styles: I) where I: IntoIterator<Item=&'a FlexStyle> {
		for style in styles {
			self.apply_style(style);
		}
	}

	pub fn apply_style(&mut self, style: &FlexStyle) {
		use FlexStyle::*;

		match *style {
			AlignContent(align) => self.set_align_content(align),
			AlignItems(align) => self.set_align_items(align),
			AlignSelf(align) => self.set_align_self(align),
			AspectRatio(a) => self.set_aspect_ratio(a.into_inner()),
			BorderBottom(w) => self.set_border(Edge::Bottom, w.into_inner()),
			BorderLeft(w) => self.set_border(Edge::Left, w.into_inner()),
			BorderRight(w) => self.set_border(Edge::Right, w.into_inner()),
			BorderTop(w) => self.set_border(Edge::Top, w.into_inner()),
			Border(w) => self.set_border(Edge::All, w.into_inner()),
			Bottom(b) => self.set_position(Edge::Bottom, b),
			End(e) => self.set_position(Edge::End, e),
			Flex(f) => self.set_flex(f.into_inner()),
			FlexBasis(f) => self.set_flex_basis(f),
			FlexGrow(f) => self.set_flex_grow(f.into_inner()),
			FlexDirection(flex_direction) => self.set_flex_direction(flex_direction),
			FlexShrink(f) => self.set_flex_shrink(f.into_inner()),
			FlexWrap(wrap) => self.set_flex_wrap(wrap),
			Height(h) => self.set_height(h),
			JustifyContent(justify) => self.set_justify_content(justify),
			Left(l) => self.set_position(Edge::Left, l),
			Margin(m) => self.set_margin(Edge::All, m),
			MarginBottom(m) => self.set_margin(Edge::Bottom, m),
			MarginHorizontal(m) => self.set_margin(Edge::Horizontal, m),
			MarginLeft(m) => self.set_margin(Edge::Left, m),
			MarginRight(m) => self.set_margin(Edge::Right, m),
			MarginTop(m) => self.set_margin(Edge::Top, m),
			MarginVertical(m) => self.set_margin(Edge::Vertical, m),
			MaxHeight(h) => self.set_max_height(h),
			MaxWidth(w) => self.set_max_width(w),
			MinHeight(h) => self.set_min_height(h),
			MinWidth(w) => self.set_min_width(w),
			Overflow(o) => self.set_overflow(o),
			Padding(p) => self.set_padding(Edge::All, p),
			PaddingBottom(p) => self.set_padding(Edge::Bottom, p),
			PaddingHorizontal(p) => self.set_padding(Edge::Horizontal, p),
			PaddingLeft(p) => self.set_padding(Edge::Left, p),
			PaddingRight(p) => self.set_padding(Edge::Right, p),
			PaddingTop(p) => self.set_padding(Edge::Top, p),
			PaddingVertical(p) => self.set_padding(Edge::Vertical, p),
			Position(position_type) => self.set_position_type(position_type),
			Right(r) => self.set_position(Edge::Right, r),
			Start(s) => self.set_position(Edge::Start, s),
			Top(t) => self.set_position(Edge::Top, t),
			Width(w) => self.set_width(w)
		}
	}

	pub fn insert_child(&mut self, child: &mut Node, index: u32) {
		let mut child = child;
		child.should_free = false;

		unsafe {
			internal::YGNodeInsertChild(self.inner_node, child.inner_node, index);
		}
	}

	pub fn child_count(&self) -> u32 {
		unsafe {
			internal::YGNodeGetChildCount(self.inner_node)
		}
	}

	pub fn set_direction(&mut self, direction: Direction) {
		unsafe {
			internal::YGNodeStyleSetDirection(self.inner_node, internal::YGDirection::from(direction));
		}
	}

	pub fn set_flex_direction(&mut self, direction: FlexDirection) {
		unsafe {
			internal::YGNodeStyleSetFlexDirection(self.inner_node, internal::YGFlexDirection::from(direction));
		}
	}

	pub fn set_justify_content(&mut self, justify: Justify) {
		unsafe {
			internal::YGNodeStyleSetJustifyContent(self.inner_node, internal::YGJustify::from(justify));
		}
	}

	pub fn set_align_content(&mut self, align: Align) {
		unsafe {
			internal::YGNodeStyleSetAlignContent(self.inner_node, internal::YGAlign::from(align));
		}
	}

	pub fn set_align_items(&mut self, align: Align) {
		unsafe {
			internal::YGNodeStyleSetAlignItems(self.inner_node, internal::YGAlign::from(align));
		}
	}

	pub fn set_align_self(&mut self, align: Align) {
		unsafe {
			internal::YGNodeStyleSetAlignSelf(self.inner_node, internal::YGAlign::from(align));
		}
	}

	pub fn set_position_type(&mut self, position_type: PositionType) {
		unsafe {
			internal::YGNodeStyleSetPositionType(self.inner_node, internal::YGPositionType::from(position_type));
		}
	}

	pub fn set_position(&mut self, edge: Edge, position: StyleUnit) {
		unsafe {
			match position {
				StyleUnit::UndefinedValue => internal::YGNodeStyleSetPosition(self.inner_node, internal::YGEdge::from(edge), Undefined),
				StyleUnit::Point(val) => internal::YGNodeStyleSetPosition(self.inner_node, internal::YGEdge::from(edge), val.into_inner()),
				StyleUnit::Percent(val) => internal::YGNodeStyleSetPositionPercent(self.inner_node, internal::YGEdge::from(edge), val.into_inner()),
				// auto is not a valid value for position
				StyleUnit::Auto => internal::YGNodeStyleSetPosition(self.inner_node, internal::YGEdge::from(edge), Undefined),
			}
		}
	}

	pub fn set_flex_wrap(&mut self, wrap: Wrap) {
		unsafe {
			internal::YGNodeStyleSetFlexWrap(self.inner_node, internal::YGWrap::from(wrap));
		}
	}

	pub fn set_overflow(&mut self, overflow: Overflow) {
		unsafe {
			internal::YGNodeStyleSetOverflow(self.inner_node, internal::YGOverflow::from(overflow));
		}
	}

	pub fn set_flex(&mut self, flex: f32) {
		unsafe {
			internal::YGNodeStyleSetFlex(self.inner_node, flex);
		}
	}

	pub fn set_flex_grow(&mut self, flex_grow: f32) {
		unsafe {
			internal::YGNodeStyleSetFlexGrow(self.inner_node, flex_grow);
		}
	}

	pub fn set_flex_shrink(&mut self, flex_shrink: f32) {
		unsafe {
			internal::YGNodeStyleSetFlexShrink(self.inner_node, flex_shrink);
		}
	}

	pub fn set_flex_basis(&mut self, flex_basis: StyleUnit) {
		unsafe {
			match flex_basis {
				StyleUnit::UndefinedValue => internal::YGNodeStyleSetFlexBasis(self.inner_node, Undefined),
				StyleUnit::Point(val) => internal::YGNodeStyleSetFlexBasis(self.inner_node, val.into_inner()),
				StyleUnit::Percent(val) => internal::YGNodeStyleSetFlexBasisPercent(self.inner_node, val.into_inner()),
				StyleUnit::Auto => internal::YGNodeStyleSetFlexBasisAuto(self.inner_node)
			}
		}
	}

	pub fn set_edge_position(&mut self, edge: Edge, position: f32) {
		unsafe {
			internal::YGNodeStyleSetPosition(self.inner_node, internal::YGEdge::from(edge), position);
		}
	}

	pub fn set_margin(&mut self, edge: Edge, margin: StyleUnit) {
		unsafe {
			match margin {
				StyleUnit::UndefinedValue => internal::YGNodeStyleSetMargin(self.inner_node, internal::YGEdge::from(edge), Undefined),
				StyleUnit::Point(val) => internal::YGNodeStyleSetMargin(self.inner_node, internal::YGEdge::from(edge), val.into_inner()),
				StyleUnit::Percent(val) => internal::YGNodeStyleSetMarginPercent(self.inner_node, internal::YGEdge::from(edge), val.into_inner()),
				StyleUnit::Auto => internal::YGNodeStyleSetMarginAuto(self.inner_node, internal::YGEdge::from(edge))
			}
		}
	}

	pub fn set_padding(&mut self, edge: Edge, padding: StyleUnit) {
		unsafe {
			match padding {
				StyleUnit::UndefinedValue => internal::YGNodeStyleSetPadding(self.inner_node, internal::YGEdge::from(edge), Undefined),
				StyleUnit::Point(val) => internal::YGNodeStyleSetPadding(self.inner_node, internal::YGEdge::from(edge), val.into_inner()),
				StyleUnit::Percent(val) => internal::YGNodeStyleSetPaddingPercent(self.inner_node, internal::YGEdge::from(edge), val.into_inner()),
				// auto is not a valid value for padding
				StyleUnit::Auto => internal::YGNodeStyleSetPadding(self.inner_node, internal::YGEdge::from(edge), Undefined)
			}
		}
	}

	pub fn set_border(&mut self, edge: Edge, border: f32) {
		unsafe {
			internal::YGNodeStyleSetBorder(self.inner_node, internal::YGEdge::from(edge), border);
		}
	}

	pub fn set_width(&mut self, width: StyleUnit) {
		unsafe {
			match width {
				StyleUnit::UndefinedValue => internal::YGNodeStyleSetWidth(self.inner_node, Undefined),
				StyleUnit::Point(val) => internal::YGNodeStyleSetWidth(self.inner_node, val.into_inner()),
				StyleUnit::Percent(val) => internal::YGNodeStyleSetWidthPercent(self.inner_node, val.into_inner()),
				StyleUnit::Auto => internal::YGNodeStyleSetWidthAuto(self.inner_node)
			}
		}
	}

	pub fn set_height(&mut self, height: StyleUnit) {
		unsafe {
			match height {
				StyleUnit::UndefinedValue => internal::YGNodeStyleSetHeight(self.inner_node, Undefined),
				StyleUnit::Point(val) => internal::YGNodeStyleSetHeight(self.inner_node, val.into_inner()),
				StyleUnit::Percent(val) => internal::YGNodeStyleSetHeightPercent(self.inner_node, val.into_inner()),
				StyleUnit::Auto => internal::YGNodeStyleSetHeightAuto(self.inner_node)
			}
		}
	}

	pub fn set_min_width(&mut self, min_width: StyleUnit) {
		unsafe {
			match min_width {
				StyleUnit::UndefinedValue => internal::YGNodeStyleSetMinWidth(self.inner_node, Undefined),
				StyleUnit::Point(val) => internal::YGNodeStyleSetMinWidth(self.inner_node, val.into_inner()),
				StyleUnit::Percent(val) => internal::YGNodeStyleSetMinWidthPercent(self.inner_node, val.into_inner()),
				// auto is not a valid value for min_width
				StyleUnit::Auto => internal::YGNodeStyleSetMinWidth(self.inner_node, Undefined),
			}
		}
	}

	pub fn set_min_height(&mut self, min_height: StyleUnit) {
		unsafe {
			match min_height {
				StyleUnit::UndefinedValue => internal::YGNodeStyleSetMinHeight(self.inner_node, Undefined),
				StyleUnit::Point(val) => internal::YGNodeStyleSetMinHeight(self.inner_node, val.into_inner()),
				StyleUnit::Percent(val) => internal::YGNodeStyleSetMinHeightPercent(self.inner_node, val.into_inner()),
				// auto is not a valid value for min_height
				StyleUnit::Auto => internal::YGNodeStyleSetMinHeight(self.inner_node, Undefined),
			}
		}
	}

	pub fn set_max_width(&mut self, max_width: StyleUnit) {
		unsafe {
			match max_width {
				StyleUnit::UndefinedValue => internal::YGNodeStyleSetMaxWidth(self.inner_node, Undefined),
				StyleUnit::Point(val) => internal::YGNodeStyleSetMaxWidth(self.inner_node, val.into_inner()),
				StyleUnit::Percent(val) => internal::YGNodeStyleSetMaxWidthPercent(self.inner_node, val.into_inner()),
				// auto is not a valid value for max_width
				StyleUnit::Auto => internal::YGNodeStyleSetMaxWidth(self.inner_node, Undefined),
			}
		}
	}

	pub fn set_max_height(&mut self, max_height: StyleUnit) {
		unsafe {
			match max_height {
				StyleUnit::UndefinedValue => internal::YGNodeStyleSetMaxHeight(self.inner_node, Undefined),
				StyleUnit::Point(val) => internal::YGNodeStyleSetMaxHeight(self.inner_node, val.into_inner()),
				StyleUnit::Percent(val) => internal::YGNodeStyleSetMaxHeightPercent(self.inner_node, val.into_inner()),
				// auto is not a valid value for max_height
				StyleUnit::Auto => internal::YGNodeStyleSetMaxHeight(self.inner_node, Undefined),
			}
		}
	}

	pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
		unsafe {
			internal::YGNodeStyleSetAspectRatio(self.inner_node, aspect_ratio);
		}
	}

	pub fn calculate_layout(&mut self, available_width: f32, available_height: f32, parent_direction: Direction) {
		unsafe {
			internal::YGNodeCalculateLayout(self.inner_node, available_width, available_height, internal::YGDirection::from(parent_direction));
		}
	}

	pub fn get_layout(&self) -> Layout {
		unsafe {
			Layout {
				left: internal::YGNodeLayoutGetLeft(self.inner_node),
				right: internal::YGNodeLayoutGetRight(self.inner_node),
				top: internal::YGNodeLayoutGetTop(self.inner_node),
				bottom: internal::YGNodeLayoutGetBottom(self.inner_node),
				width: internal::YGNodeLayoutGetWidth(self.inner_node),
				height: internal::YGNodeLayoutGetHeight(self.inner_node)
			}
		}
	}

	pub fn set_measure_func(&mut self, func: MeasureFunc) {
		match func {
			Some(f) => unsafe {
				let casted_func: InternalMeasureFunc = std::mem::transmute(f as usize);
				internal::YGNodeSetMeasureFunc(self.inner_node, Some(casted_func));
			},
			None => unsafe {
				internal::YGNodeSetMeasureFunc(self.inner_node, None);
			}
		}
	}

	pub fn set_baseline_func(&mut self, func: BaselineFunc) {
		match func {
			Some(f) => unsafe {
				let casted_func: InternalBaselineFunc = std::mem::transmute(f as usize);
				internal::YGNodeSetBaselineFunc(self.inner_node, Some(casted_func));
			},
			None => unsafe {
				internal::YGNodeSetBaselineFunc(self.inner_node, None);
			}
		}
	}
}

type InternalMeasureFunc = unsafe extern "C" fn(internal::YGNodeRef, f32, internal::YGMeasureMode, f32, internal::YGMeasureMode) -> internal::YGSize;
type InternalBaselineFunc = unsafe extern "C" fn(internal::YGNodeRef, f32, f32) -> f32;
pub type MeasureFunc = Option<extern fn(NodeRef, f32, MeasureMode, f32, MeasureMode) -> Size>;
pub type BaselineFunc = Option<extern fn(NodeRef, f32, f32) -> f32>;

impl Drop for Node {
	fn drop(&mut self) {
		if self.should_free {
			unsafe {
				internal::YGNodeFreeRecursive(self.inner_node);
			}
		}
	}
}
