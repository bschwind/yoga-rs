use crate::internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
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
            FlexDirection::RowReverse => internal::YGFlexDirection::YGFlexDirectionRowReverse,
        }
    }
}

impl From<internal::YGFlexDirection> for FlexDirection {
    fn from(f: internal::YGFlexDirection) -> FlexDirection {
        match f {
            internal::YGFlexDirection::YGFlexDirectionColumn => FlexDirection::Column,
            internal::YGFlexDirection::YGFlexDirectionColumnReverse => FlexDirection::ColumnReverse,
            internal::YGFlexDirection::YGFlexDirectionRow => FlexDirection::Row,
            internal::YGFlexDirection::YGFlexDirectionRowReverse => FlexDirection::RowReverse,
        }
    }
}
