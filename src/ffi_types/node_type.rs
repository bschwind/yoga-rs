use internal;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NodeType {
	Default = 0,
	Text = 1,
}

impl From<NodeType> for internal::YGNodeType {
	fn from(n: NodeType) -> internal::YGNodeType {
		match n {
			NodeType::Default => internal::YGNodeType::YGNodeTypeDefault,
			NodeType::Text => internal::YGNodeType::YGNodeTypeText,
		}
	}
}
