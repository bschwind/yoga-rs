use crate::internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum Gutter {
    Column = 0,
    Row = 1,
    All = 2,
}

impl From<Gutter> for internal::YGGutter {
    fn from(g: Gutter) -> internal::YGGutter {
        match g {
            Gutter::Column => internal::YGGutter::YGGutterColumn,
            Gutter::Row => internal::YGGutter::YGGutterRow,
            Gutter::All => internal::YGGutter::YGGutterAll,
        }
    }
}

impl From<internal::YGGutter> for Gutter {
    fn from(g: internal::YGGutter) -> Gutter {
        match g {
            internal::YGGutter::YGGutterColumn => Gutter::Column,
            internal::YGGutter::YGGutterRow => Gutter::Row,
            internal::YGGutter::YGGutterAll => Gutter::All,
        }
    }
}
