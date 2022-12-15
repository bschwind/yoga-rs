#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate ordered_float;
#[cfg(feature = "serde_support")]
extern crate serde;
#[macro_use]
#[cfg(feature = "serde_support")]
extern crate serde_derive;

// API created by bindgen
mod internal {
    #![allow(dead_code)]
    #![allow(clippy::enum_variant_names)] // TODO(bschwind) - Possibly change the binding logic to name enums properly.
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    pub use self::root::*;
}

// Public re-exports of Yoga enums
mod ffi_types {
    pub mod align;
    pub mod config_ref;
    pub mod dimension;
    pub mod direction;
    pub mod display;
    pub mod edge;
    pub mod flex_direction;
    pub mod justify;
    pub mod log_level;
    pub mod measure_mode;
    pub mod node_ref;
    pub mod node_type;
    pub mod overflow;
    pub mod position_type;
    pub mod print_options;
    pub mod size;
    pub mod style_unit;
    pub mod undefined;
    pub mod wrap;
}

pub mod prelude;
pub mod traits;
pub mod types;

pub use crate::types::*;
use std::any::Any;

#[repr(C)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Config {
    inner_config: ConfigRef,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node {
    inner_node: NodeRef,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Config {
        Config { inner_config: unsafe { internal::YGConfigNew() } }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

impl Node {
    pub fn new() -> Node {
        Node { inner_node: unsafe { internal::YGNodeNew() } }
    }

    pub fn new_with_config(config: &mut Config) -> Node {
        Node { inner_node: unsafe { internal::YGNodeNewWithConfig(config.inner_config) } }
    }

    pub fn reset(&mut self) {
        unsafe {
            internal::YGNodeReset(self.inner_node);
        }
    }

    pub fn mark_dirty(&mut self) {
        unsafe {
            internal::YGNodeMarkDirty(self.inner_node);
        }
    }

    pub fn apply_styles<'a, I>(&mut self, styles: I)
    where
        I: IntoIterator<Item = &'a FlexStyle>,
    {
        for style in styles {
            self.apply_style(style);
        }
    }

    pub fn apply_style(&mut self, style: &FlexStyle) {
        use crate::FlexStyle::*;

        match *style {
            AlignContent(align) => self.set_align_content(align),
            AlignItems(align) => self.set_align_items(align),
            AlignSelf(align) => self.set_align_self(align),
            AspectRatio(a) => self.set_aspect_ratio(a.into_inner()),
            BorderBottom(b) => self.set_border(Edge::Bottom, b.into_inner()),
            BorderEnd(b) => self.set_border(Edge::End, b.into_inner()),
            BorderLeft(b) => self.set_border(Edge::Left, b.into_inner()),
            BorderRight(b) => self.set_border(Edge::Right, b.into_inner()),
            BorderStart(b) => self.set_border(Edge::Start, b.into_inner()),
            BorderTop(b) => self.set_border(Edge::Top, b.into_inner()),
            Border(b) => self.set_border(Edge::All, b.into_inner()),
            Bottom(b) => self.set_position(Edge::Bottom, b),
            Display(d) => self.set_display(d),
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
            MarginEnd(m) => self.set_margin(Edge::End, m),
            MarginHorizontal(m) => self.set_margin(Edge::Horizontal, m),
            MarginLeft(m) => self.set_margin(Edge::Left, m),
            MarginRight(m) => self.set_margin(Edge::Right, m),
            MarginStart(m) => self.set_margin(Edge::Start, m),
            MarginTop(m) => self.set_margin(Edge::Top, m),
            MarginVertical(m) => self.set_margin(Edge::Vertical, m),
            MaxHeight(h) => self.set_max_height(h),
            MaxWidth(w) => self.set_max_width(w),
            MinHeight(h) => self.set_min_height(h),
            MinWidth(w) => self.set_min_width(w),
            Overflow(o) => self.set_overflow(o),
            Padding(p) => self.set_padding(Edge::All, p),
            PaddingBottom(p) => self.set_padding(Edge::Bottom, p),
            PaddingEnd(p) => self.set_padding(Edge::End, p),
            PaddingHorizontal(p) => self.set_padding(Edge::Horizontal, p),
            PaddingLeft(p) => self.set_padding(Edge::Left, p),
            PaddingRight(p) => self.set_padding(Edge::Right, p),
            PaddingStart(p) => self.set_padding(Edge::Start, p),
            PaddingTop(p) => self.set_padding(Edge::Top, p),
            PaddingVertical(p) => self.set_padding(Edge::Vertical, p),
            Position(position_type) => self.set_position_type(position_type),
            Right(r) => self.set_position(Edge::Right, r),
            Start(s) => self.set_position(Edge::Start, s),
            Top(t) => self.set_position(Edge::Top, t),
            Width(w) => self.set_width(w),
        }
    }

    pub fn insert_child(&mut self, child: &mut Node, index: u32) {
        unsafe {
            internal::YGNodeInsertChild(self.inner_node, child.inner_node, index);
        }
    }

    pub fn remove_child(&mut self, child: &mut Node) {
        unsafe {
            internal::YGNodeRemoveChild(self.inner_node, child.inner_node);
        }
    }

    pub fn child_count(&self) -> u32 {
        unsafe { internal::YGNodeGetChildCount(self.inner_node) }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        unsafe {
            internal::YGNodeStyleSetDirection(
                self.inner_node,
                internal::YGDirection::from(direction),
            );
        }
    }

    pub fn set_flex_direction(&mut self, direction: FlexDirection) {
        unsafe {
            internal::YGNodeStyleSetFlexDirection(
                self.inner_node,
                internal::YGFlexDirection::from(direction),
            );
        }
    }

    pub fn set_justify_content(&mut self, justify: Justify) {
        unsafe {
            internal::YGNodeStyleSetJustifyContent(
                self.inner_node,
                internal::YGJustify::from(justify),
            );
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
            internal::YGNodeStyleSetPositionType(
                self.inner_node,
                internal::YGPositionType::from(position_type),
            );
        }
    }

    pub fn set_position(&mut self, edge: Edge, position: StyleUnit) {
        unsafe {
            match position {
                StyleUnit::UndefinedValue => internal::YGNodeStyleSetPosition(
                    self.inner_node,
                    internal::YGEdge::from(edge),
                    Undefined,
                ),
                StyleUnit::Point(val) => internal::YGNodeStyleSetPosition(
                    self.inner_node,
                    internal::YGEdge::from(edge),
                    val.into_inner(),
                ),
                StyleUnit::Percent(val) => internal::YGNodeStyleSetPositionPercent(
                    self.inner_node,
                    internal::YGEdge::from(edge),
                    val.into_inner(),
                ),
                // auto is not a valid value for position
                StyleUnit::Auto => internal::YGNodeStyleSetPosition(
                    self.inner_node,
                    internal::YGEdge::from(edge),
                    Undefined,
                ),
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
                StyleUnit::UndefinedValue => {
                    internal::YGNodeStyleSetFlexBasis(self.inner_node, Undefined)
                },
                StyleUnit::Point(val) => {
                    internal::YGNodeStyleSetFlexBasis(self.inner_node, val.into_inner())
                },
                StyleUnit::Percent(val) => {
                    internal::YGNodeStyleSetFlexBasisPercent(self.inner_node, val.into_inner())
                },
                StyleUnit::Auto => internal::YGNodeStyleSetFlexBasisAuto(self.inner_node),
            }
        }
    }

    pub fn set_edge_position(&mut self, edge: Edge, position: f32) {
        unsafe {
            internal::YGNodeStyleSetPosition(
                self.inner_node,
                internal::YGEdge::from(edge),
                position,
            );
        }
    }

    pub fn set_margin(&mut self, edge: Edge, margin: StyleUnit) {
        unsafe {
            match margin {
                StyleUnit::UndefinedValue => internal::YGNodeStyleSetMargin(
                    self.inner_node,
                    internal::YGEdge::from(edge),
                    Undefined,
                ),
                StyleUnit::Point(val) => internal::YGNodeStyleSetMargin(
                    self.inner_node,
                    internal::YGEdge::from(edge),
                    val.into_inner(),
                ),
                StyleUnit::Percent(val) => internal::YGNodeStyleSetMarginPercent(
                    self.inner_node,
                    internal::YGEdge::from(edge),
                    val.into_inner(),
                ),
                StyleUnit::Auto => internal::YGNodeStyleSetMarginAuto(
                    self.inner_node,
                    internal::YGEdge::from(edge),
                ),
            }
        }
    }

    pub fn set_padding(&mut self, edge: Edge, padding: StyleUnit) {
        unsafe {
            match padding {
                StyleUnit::UndefinedValue => internal::YGNodeStyleSetPadding(
                    self.inner_node,
                    internal::YGEdge::from(edge),
                    Undefined,
                ),
                StyleUnit::Point(val) => internal::YGNodeStyleSetPadding(
                    self.inner_node,
                    internal::YGEdge::from(edge),
                    val.into_inner(),
                ),
                StyleUnit::Percent(val) => internal::YGNodeStyleSetPaddingPercent(
                    self.inner_node,
                    internal::YGEdge::from(edge),
                    val.into_inner(),
                ),
                // auto is not a valid value for padding
                StyleUnit::Auto => internal::YGNodeStyleSetPadding(
                    self.inner_node,
                    internal::YGEdge::from(edge),
                    Undefined,
                ),
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
                StyleUnit::UndefinedValue => {
                    internal::YGNodeStyleSetWidth(self.inner_node, Undefined)
                },
                StyleUnit::Point(val) => {
                    internal::YGNodeStyleSetWidth(self.inner_node, val.into_inner())
                },
                StyleUnit::Percent(val) => {
                    internal::YGNodeStyleSetWidthPercent(self.inner_node, val.into_inner())
                },
                StyleUnit::Auto => internal::YGNodeStyleSetWidthAuto(self.inner_node),
            }
        }
    }

    pub fn set_height(&mut self, height: StyleUnit) {
        unsafe {
            match height {
                StyleUnit::UndefinedValue => {
                    internal::YGNodeStyleSetHeight(self.inner_node, Undefined)
                },
                StyleUnit::Point(val) => {
                    internal::YGNodeStyleSetHeight(self.inner_node, val.into_inner())
                },
                StyleUnit::Percent(val) => {
                    internal::YGNodeStyleSetHeightPercent(self.inner_node, val.into_inner())
                },
                StyleUnit::Auto => internal::YGNodeStyleSetHeightAuto(self.inner_node),
            }
        }
    }

    pub fn set_min_width(&mut self, min_width: StyleUnit) {
        unsafe {
            match min_width {
                StyleUnit::UndefinedValue => {
                    internal::YGNodeStyleSetMinWidth(self.inner_node, Undefined)
                },
                StyleUnit::Point(val) => {
                    internal::YGNodeStyleSetMinWidth(self.inner_node, val.into_inner())
                },
                StyleUnit::Percent(val) => {
                    internal::YGNodeStyleSetMinWidthPercent(self.inner_node, val.into_inner())
                },
                // auto is not a valid value for min_width
                StyleUnit::Auto => internal::YGNodeStyleSetMinWidth(self.inner_node, Undefined),
            }
        }
    }

    pub fn set_min_height(&mut self, min_height: StyleUnit) {
        unsafe {
            match min_height {
                StyleUnit::UndefinedValue => {
                    internal::YGNodeStyleSetMinHeight(self.inner_node, Undefined)
                },
                StyleUnit::Point(val) => {
                    internal::YGNodeStyleSetMinHeight(self.inner_node, val.into_inner())
                },
                StyleUnit::Percent(val) => {
                    internal::YGNodeStyleSetMinHeightPercent(self.inner_node, val.into_inner())
                },
                // auto is not a valid value for min_height
                StyleUnit::Auto => internal::YGNodeStyleSetMinHeight(self.inner_node, Undefined),
            }
        }
    }

    pub fn set_max_width(&mut self, max_width: StyleUnit) {
        unsafe {
            match max_width {
                StyleUnit::UndefinedValue => {
                    internal::YGNodeStyleSetMaxWidth(self.inner_node, Undefined)
                },
                StyleUnit::Point(val) => {
                    internal::YGNodeStyleSetMaxWidth(self.inner_node, val.into_inner())
                },
                StyleUnit::Percent(val) => {
                    internal::YGNodeStyleSetMaxWidthPercent(self.inner_node, val.into_inner())
                },
                // auto is not a valid value for max_width
                StyleUnit::Auto => internal::YGNodeStyleSetMaxWidth(self.inner_node, Undefined),
            }
        }
    }

    pub fn set_max_height(&mut self, max_height: StyleUnit) {
        unsafe {
            match max_height {
                StyleUnit::UndefinedValue => {
                    internal::YGNodeStyleSetMaxHeight(self.inner_node, Undefined)
                },
                StyleUnit::Point(val) => {
                    internal::YGNodeStyleSetMaxHeight(self.inner_node, val.into_inner())
                },
                StyleUnit::Percent(val) => {
                    internal::YGNodeStyleSetMaxHeightPercent(self.inner_node, val.into_inner())
                },
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

    pub fn calculate_layout(
        &mut self,
        available_width: f32,
        available_height: f32,
        parent_direction: Direction,
    ) {
        unsafe {
            internal::YGNodeCalculateLayout(
                self.inner_node,
                available_width,
                available_height,
                internal::YGDirection::from(parent_direction),
            );
        }
    }

    pub fn get_layout(&self) -> Layout {
        unsafe {
            Layout::new(
                internal::YGNodeLayoutGetLeft(self.inner_node),
                internal::YGNodeLayoutGetRight(self.inner_node),
                internal::YGNodeLayoutGetTop(self.inner_node),
                internal::YGNodeLayoutGetBottom(self.inner_node),
                internal::YGNodeLayoutGetWidth(self.inner_node),
                internal::YGNodeLayoutGetHeight(self.inner_node),
            )
        }
    }

    pub fn get_child_count(&self) -> u32 {
        unsafe { internal::YGNodeGetChildCount(self.inner_node) }
    }

    pub fn get_child(&self, index: u32) -> NodeRef {
        unsafe { internal::YGNodeGetChild(self.inner_node, index) }
    }

    pub fn get_style_direction(&self) -> Direction {
        unsafe { internal::YGNodeStyleGetDirection(self.inner_node).into() }
    }

    pub fn get_flex_direction(&self) -> FlexDirection {
        unsafe { internal::YGNodeStyleGetFlexDirection(self.inner_node).into() }
    }

    pub fn get_justify_content(&self) -> Justify {
        unsafe { internal::YGNodeStyleGetJustifyContent(self.inner_node).into() }
    }

    pub fn get_align_content(&self) -> Align {
        unsafe { internal::YGNodeStyleGetAlignContent(self.inner_node).into() }
    }

    pub fn get_align_items(&self) -> Align {
        unsafe { internal::YGNodeStyleGetAlignItems(self.inner_node).into() }
    }

    pub fn get_align_self(&self) -> Align {
        unsafe { internal::YGNodeStyleGetAlignSelf(self.inner_node).into() }
    }

    pub fn get_position_type(&self) -> PositionType {
        unsafe { internal::YGNodeStyleGetPositionType(self.inner_node).into() }
    }

    pub fn get_flex_wrap(&self) -> Wrap {
        unsafe { internal::YGNodeStyleGetFlexWrap(self.inner_node).into() }
    }

    pub fn get_overflow(&self) -> Overflow {
        unsafe { internal::YGNodeStyleGetOverflow(self.inner_node).into() }
    }

    pub fn get_flex_grow(&self) -> f32 {
        unsafe { internal::YGNodeStyleGetFlexGrow(self.inner_node) }
    }

    pub fn get_flex_shrink(&self) -> f32 {
        unsafe { internal::YGNodeStyleGetFlexShrink(self.inner_node) }
    }

    pub fn get_flex_basis(&self) -> StyleUnit {
        unsafe { internal::YGNodeStyleGetFlexBasis(self.inner_node).into() }
    }

    pub fn get_style_position_left(&self) -> StyleUnit {
        unsafe {
            internal::YGNodeStyleGetPosition(self.inner_node, internal::YGEdge::from(Edge::Left))
                .into()
        }
    }

    pub fn get_style_position_right(&self) -> StyleUnit {
        unsafe {
            internal::YGNodeStyleGetPosition(self.inner_node, internal::YGEdge::from(Edge::Right))
                .into()
        }
    }

    pub fn get_style_position_top(&self) -> StyleUnit {
        unsafe {
            internal::YGNodeStyleGetPosition(self.inner_node, internal::YGEdge::from(Edge::Top))
                .into()
        }
    }

    pub fn get_style_position_bottom(&self) -> StyleUnit {
        unsafe {
            internal::YGNodeStyleGetPosition(self.inner_node, internal::YGEdge::from(Edge::Bottom))
                .into()
        }
    }

    pub fn get_style_position_start(&self) -> StyleUnit {
        unsafe {
            internal::YGNodeStyleGetPosition(self.inner_node, internal::YGEdge::from(Edge::Start))
                .into()
        }
    }

    pub fn get_style_position_end(&self) -> StyleUnit {
        unsafe {
            internal::YGNodeStyleGetPosition(self.inner_node, internal::YGEdge::from(Edge::End))
                .into()
        }
    }

    pub fn get_style_margin_left(&self) -> StyleUnit {
        unsafe {
            internal::YGNodeStyleGetMargin(self.inner_node, internal::YGEdge::from(Edge::Left))
                .into()
        }
    }

    pub fn get_style_margin_right(&self) -> StyleUnit {
        unsafe {
            internal::YGNodeStyleGetMargin(self.inner_node, internal::YGEdge::from(Edge::Right))
                .into()
        }
    }

    pub fn get_style_margin_top(&self) -> StyleUnit {
        unsafe {
            internal::YGNodeStyleGetMargin(self.inner_node, internal::YGEdge::from(Edge::Top))
                .into()
        }
    }

    pub fn get_style_margin_bottom(&self) -> StyleUnit {
        unsafe {
            internal::YGNodeStyleGetMargin(self.inner_node, internal::YGEdge::from(Edge::Bottom))
                .into()
        }
    }

    pub fn get_style_margin_start(&self) -> StyleUnit {
        unsafe {
            internal::YGNodeStyleGetMargin(self.inner_node, internal::YGEdge::from(Edge::Start))
                .into()
        }
    }

    pub fn get_style_margin_end(&self) -> StyleUnit {
        unsafe {
            internal::YGNodeStyleGetMargin(self.inner_node, internal::YGEdge::from(Edge::End))
                .into()
        }
    }

    pub fn get_style_padding_left(&self) -> StyleUnit {
        unsafe {
            internal::YGNodeStyleGetPadding(self.inner_node, internal::YGEdge::from(Edge::Left))
                .into()
        }
    }

    pub fn get_style_padding_right(&self) -> StyleUnit {
        unsafe {
            internal::YGNodeStyleGetPadding(self.inner_node, internal::YGEdge::from(Edge::Right))
                .into()
        }
    }

    pub fn get_style_padding_top(&self) -> StyleUnit {
        unsafe {
            internal::YGNodeStyleGetPadding(self.inner_node, internal::YGEdge::from(Edge::Top))
                .into()
        }
    }

    pub fn get_style_padding_bottom(&self) -> StyleUnit {
        unsafe {
            internal::YGNodeStyleGetPadding(self.inner_node, internal::YGEdge::from(Edge::Bottom))
                .into()
        }
    }

    pub fn get_style_padding_start(&self) -> StyleUnit {
        unsafe {
            internal::YGNodeStyleGetPadding(self.inner_node, internal::YGEdge::from(Edge::Start))
                .into()
        }
    }

    pub fn get_style_padding_end(&self) -> StyleUnit {
        unsafe {
            internal::YGNodeStyleGetPadding(self.inner_node, internal::YGEdge::from(Edge::End))
                .into()
        }
    }

    pub fn get_style_border_left(&self) -> f32 {
        unsafe {
            internal::YGNodeStyleGetBorder(self.inner_node, internal::YGEdge::from(Edge::Left))
        }
    }

    pub fn get_style_border_right(&self) -> f32 {
        unsafe {
            internal::YGNodeStyleGetBorder(self.inner_node, internal::YGEdge::from(Edge::Right))
        }
    }

    pub fn get_style_border_top(&self) -> f32 {
        unsafe {
            internal::YGNodeStyleGetBorder(self.inner_node, internal::YGEdge::from(Edge::Top))
        }
    }

    pub fn get_style_border_bottom(&self) -> f32 {
        unsafe {
            internal::YGNodeStyleGetBorder(self.inner_node, internal::YGEdge::from(Edge::Bottom))
        }
    }

    pub fn get_style_border_start(&self) -> f32 {
        unsafe {
            internal::YGNodeStyleGetBorder(self.inner_node, internal::YGEdge::from(Edge::Start))
        }
    }

    pub fn get_style_border_end(&self) -> f32 {
        unsafe {
            internal::YGNodeStyleGetBorder(self.inner_node, internal::YGEdge::from(Edge::End))
        }
    }

    pub fn get_style_width(&self) -> StyleUnit {
        unsafe { internal::YGNodeStyleGetWidth(self.inner_node).into() }
    }

    pub fn get_style_height(&self) -> StyleUnit {
        unsafe { internal::YGNodeStyleGetHeight(self.inner_node).into() }
    }

    pub fn get_style_min_width(&self) -> StyleUnit {
        unsafe { internal::YGNodeStyleGetMinWidth(self.inner_node).into() }
    }

    pub fn get_style_min_height(&self) -> StyleUnit {
        unsafe { internal::YGNodeStyleGetMinHeight(self.inner_node).into() }
    }

    pub fn get_style_max_width(&self) -> StyleUnit {
        unsafe { internal::YGNodeStyleGetMaxWidth(self.inner_node).into() }
    }

    pub fn get_style_max_height(&self) -> StyleUnit {
        unsafe { internal::YGNodeStyleGetMaxHeight(self.inner_node).into() }
    }

    // Layout Getters
    pub fn get_layout_margin_left(&self) -> f32 {
        unsafe {
            internal::YGNodeLayoutGetMargin(self.inner_node, internal::YGEdge::from(Edge::Left))
        }
    }

    pub fn get_layout_margin_right(&self) -> f32 {
        unsafe {
            internal::YGNodeLayoutGetMargin(self.inner_node, internal::YGEdge::from(Edge::Right))
        }
    }

    pub fn get_layout_margin_top(&self) -> f32 {
        unsafe {
            internal::YGNodeLayoutGetMargin(self.inner_node, internal::YGEdge::from(Edge::Top))
        }
    }

    pub fn get_layout_margin_bottom(&self) -> f32 {
        unsafe {
            internal::YGNodeLayoutGetMargin(self.inner_node, internal::YGEdge::from(Edge::Bottom))
        }
    }

    pub fn get_layout_margin_start(&self) -> f32 {
        unsafe {
            internal::YGNodeLayoutGetMargin(self.inner_node, internal::YGEdge::from(Edge::Start))
        }
    }

    pub fn get_layout_margin_end(&self) -> f32 {
        unsafe {
            internal::YGNodeLayoutGetMargin(self.inner_node, internal::YGEdge::from(Edge::End))
        }
    }

    pub fn get_layout_padding_left(&self) -> f32 {
        unsafe {
            internal::YGNodeLayoutGetPadding(self.inner_node, internal::YGEdge::from(Edge::Left))
        }
    }

    pub fn get_layout_padding_right(&self) -> f32 {
        unsafe {
            internal::YGNodeLayoutGetPadding(self.inner_node, internal::YGEdge::from(Edge::Right))
        }
    }

    pub fn get_layout_padding_top(&self) -> f32 {
        unsafe {
            internal::YGNodeLayoutGetPadding(self.inner_node, internal::YGEdge::from(Edge::Top))
        }
    }

    pub fn get_layout_padding_bottom(&self) -> f32 {
        unsafe {
            internal::YGNodeLayoutGetPadding(self.inner_node, internal::YGEdge::from(Edge::Bottom))
        }
    }

    pub fn get_layout_padding_start(&self) -> f32 {
        unsafe {
            internal::YGNodeLayoutGetPadding(self.inner_node, internal::YGEdge::from(Edge::Start))
        }
    }

    pub fn get_layout_padding_end(&self) -> f32 {
        unsafe {
            internal::YGNodeLayoutGetPadding(self.inner_node, internal::YGEdge::from(Edge::End))
        }
    }

    pub fn get_layout_left(&self) -> f32 {
        unsafe { internal::YGNodeLayoutGetLeft(self.inner_node) }
    }

    pub fn get_layout_right(&self) -> f32 {
        unsafe { internal::YGNodeLayoutGetRight(self.inner_node) }
    }

    pub fn get_layout_top(&self) -> f32 {
        unsafe { internal::YGNodeLayoutGetTop(self.inner_node) }
    }

    pub fn get_layout_bottom(&self) -> f32 {
        unsafe { internal::YGNodeLayoutGetBottom(self.inner_node) }
    }

    pub fn get_layout_border_left(&self) -> f32 {
        unsafe {
            internal::YGNodeLayoutGetBorder(self.inner_node, internal::YGEdge::from(Edge::Left))
        }
    }

    pub fn get_layout_border_right(&self) -> f32 {
        unsafe {
            internal::YGNodeLayoutGetBorder(self.inner_node, internal::YGEdge::from(Edge::Right))
        }
    }

    pub fn get_layout_border_top(&self) -> f32 {
        unsafe {
            internal::YGNodeLayoutGetBorder(self.inner_node, internal::YGEdge::from(Edge::Top))
        }
    }

    pub fn get_layout_border_bottom(&self) -> f32 {
        unsafe {
            internal::YGNodeLayoutGetBorder(self.inner_node, internal::YGEdge::from(Edge::Bottom))
        }
    }

    pub fn get_layout_width(&self) -> f32 {
        unsafe { internal::YGNodeLayoutGetWidth(self.inner_node) }
    }

    pub fn get_layout_height(&self) -> f32 {
        unsafe { internal::YGNodeLayoutGetHeight(self.inner_node) }
    }

    pub fn get_layout_direction(&self) -> Direction {
        unsafe { internal::YGNodeLayoutGetDirection(self.inner_node).into() }
    }

    pub fn is_dirty(&self) -> bool {
        unsafe { internal::YGNodeIsDirty(self.inner_node) }
    }

    pub fn copy_style(&self, src_node: &Node) {
        unsafe { internal::YGNodeCopyStyle(self.inner_node, src_node.inner_node) }
    }

    pub fn set_display(&mut self, display: Display) {
        unsafe {
            internal::YGNodeStyleSetDisplay(self.inner_node, display.into());
        }
    }

    pub fn set_measure_func(&mut self, func: MeasureFunc) {
        match func {
            Some(f) => unsafe {
                type Callback = unsafe extern "C" fn(
                    internal::YGNodeRef,
                    f32,
                    internal::YGMeasureMode,
                    f32,
                    internal::YGMeasureMode,
                ) -> internal::YGSize;
                let casted_func: Callback = std::mem::transmute(f as usize);
                internal::YGNodeSetMeasureFunc(self.inner_node, Some(casted_func));
            },
            None => unsafe {
                internal::YGNodeSetMeasureFunc(self.inner_node, None);
            },
        }
    }

    pub fn set_baseline_func(&mut self, func: BaselineFunc) {
        match func {
            Some(f) => unsafe {
                type Callback = unsafe extern "C" fn(internal::YGNodeRef, f32, f32) -> f32;
                let casted_func: Callback = std::mem::transmute(f as usize);
                internal::YGNodeSetBaselineFunc(self.inner_node, Some(casted_func));
            },
            None => unsafe {
                internal::YGNodeSetBaselineFunc(self.inner_node, None);
            },
        }
    }

    pub fn set_context(&mut self, value: Option<Context>) {
        self.drop_context();

        let raw = value.map_or_else(std::ptr::null_mut, |context| context.into_raw());
        unsafe { internal::YGNodeSetContext(self.inner_node, raw) }
    }

    pub fn get_context(node_ref: &NodeRef) -> Option<&Box<dyn Any>> {
        let raw = unsafe { internal::YGNodeGetContext(*node_ref) };
        Context::get_inner_ref(raw)
    }

    pub fn get_context_mut(node_ref: &NodeRef) -> Option<&mut Box<dyn Any>> {
        let raw = unsafe { internal::YGNodeGetContext(*node_ref) };
        Context::get_inner_mut(raw)
    }

    pub fn get_own_context(&self) -> Option<&Box<dyn Any>> {
        Node::get_context(&self.inner_node)
    }

    pub fn get_own_context_mut(&self) -> Option<&mut Box<dyn Any>> {
        Node::get_context_mut(&self.inner_node)
    }

    pub fn drop_context(&mut self) {
        let prev_raw = unsafe { internal::YGNodeGetContext(self.inner_node) };
        Context::drop_raw(prev_raw);
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        self.set_context(None);

        unsafe {
            // In the current revision (a20bde8444474e7a34352a78073de23c26e08fc5),
            // YGNodeFree does not mark the parent as dirty, but YGNodeRemoveChild does.
            // TODO remove the following lines when upgrading to a more recent revision of yoga.
            let parent = internal::YGNodeGetParent(self.inner_node);
            if parent != 0 as NodeRef {
                internal::YGNodeRemoveChild(
                    internal::YGNodeGetParent(self.inner_node),
                    self.inner_node,
                );
            }

            internal::YGNodeFree(self.inner_node);
        }
    }
}
