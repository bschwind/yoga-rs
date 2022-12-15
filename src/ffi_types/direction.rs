use crate::internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum Direction {
    Inherit = 0,
    LTR = 1,
    RTL = 2,
}

impl From<Direction> for internal::YGDirection {
    fn from(d: Direction) -> internal::YGDirection {
        match d {
            Direction::Inherit => internal::YGDirection::YGDirectionInherit,
            Direction::LTR => internal::YGDirection::YGDirectionLTR,
            Direction::RTL => internal::YGDirection::YGDirectionRTL,
        }
    }
}

impl From<internal::YGDirection> for Direction {
    fn from(d: internal::YGDirection) -> Direction {
        match d {
            internal::YGDirection::YGDirectionInherit => Direction::Inherit,
            internal::YGDirection::YGDirectionLTR => Direction::LTR,
            internal::YGDirection::YGDirectionRTL => Direction::RTL,
        }
    }
}
