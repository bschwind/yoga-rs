#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// API created by bindgen
#![allow(dead_code)]
mod internal {
	include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// Public re-exports of Yoga enums
pub use internal::YGAlign as Align;
pub use internal::YGDimension as Dimension;
pub use internal::YGDirection as Direction;
pub use internal::YGDisplay as FlexDisplay;
pub use internal::YGEdge as Edge;
pub use internal::YGExperimentalFeature;
pub use internal::YGFlexDirection as FlexDirection;
pub use internal::YGJustify as Justify;
pub use internal::YGLogLevel;
pub use internal::YGMeasureMode as MeasureMode;
pub use internal::YGNodeType;
pub use internal::YGOverflow as Overflow;
pub use internal::YGPositionType as PositionType;
pub use internal::YGPrintOptions;
pub use internal::YGUnit;
pub use internal::YGWrap as Wrap;

// Custom Rust API

#[derive(Debug)]
pub struct Node {
	inner_node: internal::YGNodeRef,
	should_free: bool
}

#[derive(Debug)]
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

	pub fn insert_child(&mut self, child: Node, index: u32) {
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
			internal::YGNodeStyleSetDirection(self.inner_node, direction);
		}
	}

	pub fn set_flex_direction(&mut self, direction: FlexDirection) {
		unsafe {
			internal::YGNodeStyleSetFlexDirection(self.inner_node, direction);
		}
	}

	pub fn set_justify_content(&mut self, justify: Justify) {
		unsafe {
			internal::YGNodeStyleSetJustifyContent(self.inner_node, justify);
		}
	}

	pub fn set_align_content(&mut self, align: Align) {
		unsafe {
			internal::YGNodeStyleSetAlignContent(self.inner_node, align);
		}
	}

	pub fn set_align_items(&mut self, align: Align) {
		unsafe {
			internal::YGNodeStyleSetAlignItems(self.inner_node, align);
		}
	}

	pub fn set_align_self(&mut self, align: Align) {
		unsafe {
			internal::YGNodeStyleSetAlignSelf(self.inner_node, align);
		}
	}

	pub fn set_position_type(&mut self, position_type: PositionType) {
		unsafe {
			internal::YGNodeStyleSetPositionType(self.inner_node, position_type);
		}
	}

	pub fn set_flex_wrap(&mut self, wrap: Wrap) {
		unsafe {
			internal::YGNodeStyleSetFlexWrap(self.inner_node, wrap);
		}
	}

	pub fn set_overflow(&mut self, overflow: Overflow) {
		unsafe {
			internal::YGNodeStyleSetOverflow(self.inner_node, overflow);
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

	pub fn set_flex_basis(&mut self, flex_basis: f32) {
		unsafe {
			internal::YGNodeStyleSetFlexBasis(self.inner_node, flex_basis);
		}
	}

	pub fn set_edge_position(&mut self, edge: Edge, position: f32) {
		unsafe {
			internal::YGNodeStyleSetPosition(self.inner_node, edge, position);
		}
	}

	pub fn set_margin(&mut self, edge: Edge, margin: f32) {
		unsafe {
			internal::YGNodeStyleSetMargin(self.inner_node, edge, margin);
		}
	}

	pub fn set_padding(&mut self, edge: Edge, padding: f32) {
		unsafe {
			internal::YGNodeStyleSetPadding(self.inner_node, edge, padding);
		}
	}

	pub fn set_border(&mut self, edge: Edge, border: f32) {
		unsafe {
			internal::YGNodeStyleSetBorder(self.inner_node, edge, border);
		}
	}

	pub fn set_width(&mut self, width: f32) {
		unsafe {
			internal::YGNodeStyleSetWidth(self.inner_node, width);
		}
	}

	pub fn set_height(&mut self, height: f32) {
		unsafe {
			internal::YGNodeStyleSetHeight(self.inner_node, height);
		}
	}

	pub fn set_min_width(&mut self, min_width: f32) {
		unsafe {
			internal::YGNodeStyleSetMinWidth(self.inner_node, min_width);
		}
	}

	pub fn set_min_height(&mut self, min_height: f32) {
		unsafe {
			internal::YGNodeStyleSetMinHeight(self.inner_node, min_height);
		}
	}

	pub fn set_max_width(&mut self, max_width: f32) {
		unsafe {
			internal::YGNodeStyleSetMaxWidth(self.inner_node, max_width);
		}
	}

	pub fn set_max_height(&mut self, max_height: f32) {
		unsafe {
			internal::YGNodeStyleSetMaxHeight(self.inner_node, max_height);
		}
	}

	pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
		unsafe {
			internal::YGNodeStyleSetAspectRatio(self.inner_node, aspect_ratio);
		}
	}

	pub fn calculate_layout(&mut self, available_width: f32, available_height: f32, parent_direction: Direction) {
		unsafe {
			internal::YGNodeCalculateLayout(self.inner_node, available_width, available_height, parent_direction);
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
}

impl Drop for Node {
	fn drop(&mut self) {
		if self.should_free {
			unsafe {
				internal::YGNodeFreeRecursive(self.inner_node);
			}
		}		
	}
}
