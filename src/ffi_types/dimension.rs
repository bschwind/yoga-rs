use crate::internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum Dimension {
    Width = 0,
    Height = 1,
}

impl From<Dimension> for internal::YGDimension {
    fn from(d: Dimension) -> internal::YGDimension {
        match d {
            Dimension::Width => internal::YGDimension::YGDimensionWidth,
            Dimension::Height => internal::YGDimension::YGDimensionHeight,
        }
    }
}
