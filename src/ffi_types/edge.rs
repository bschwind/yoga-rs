use crate::internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum Edge {
    Left = 0,
    Top = 1,
    Right = 2,
    Bottom = 3,
    Start = 4,
    End = 5,
    Horizontal = 6,
    Vertical = 7,
    All = 8,
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
            Edge::All => internal::YGEdge::YGEdgeAll,
        }
    }
}
