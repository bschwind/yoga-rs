use crate::internal;
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum StyleUnit {
    UndefinedValue,
    Point(OrderedFloat<f32>),
    Percent(OrderedFloat<f32>),
    Auto,
}

impl From<StyleUnit> for internal::YGUnit {
    fn from(s: StyleUnit) -> internal::YGUnit {
        match s {
            StyleUnit::UndefinedValue => internal::YGUnit::Undefined,
            StyleUnit::Point(_) => internal::YGUnit::Point,
            StyleUnit::Percent(_) => internal::YGUnit::Percent,
            StyleUnit::Auto => internal::YGUnit::Auto,
        }
    }
}

impl From<internal::YGValue> for StyleUnit {
    fn from(v: internal::YGValue) -> StyleUnit {
        match v.unit {
            internal::YGUnit::Undefined => StyleUnit::UndefinedValue,
            internal::YGUnit::Point => StyleUnit::Point(OrderedFloat(v.value)),
            internal::YGUnit::Percent => StyleUnit::Percent(OrderedFloat(v.value)),
            internal::YGUnit::Auto => StyleUnit::Auto,
        }
    }
}
