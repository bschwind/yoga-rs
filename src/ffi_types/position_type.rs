use crate::internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum PositionType {
    Relative = 0,
    Absolute = 1,
}

impl From<PositionType> for internal::YGPositionType {
    fn from(p: PositionType) -> internal::YGPositionType {
        match p {
            PositionType::Relative => internal::YGPositionType::YGPositionTypeRelative,
            PositionType::Absolute => internal::YGPositionType::YGPositionTypeAbsolute,
        }
    }
}

impl From<internal::YGPositionType> for PositionType {
    fn from(p: internal::YGPositionType) -> PositionType {
        match p {
            internal::YGPositionType::YGPositionTypeRelative => PositionType::Relative,
            internal::YGPositionType::YGPositionTypeAbsolute => PositionType::Absolute,
        }
    }
}
