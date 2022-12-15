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
            StyleUnit::UndefinedValue => internal::YGUnit::YGUnitUndefined,
            StyleUnit::Point(_) => internal::YGUnit::YGUnitPoint,
            StyleUnit::Percent(_) => internal::YGUnit::YGUnitPercent,
            StyleUnit::Auto => internal::YGUnit::YGUnitAuto,
        }
    }
}

impl From<internal::YGValue> for StyleUnit {
    fn from(v: internal::YGValue) -> StyleUnit {
        match v.unit {
            internal::YGUnit::YGUnitUndefined => StyleUnit::UndefinedValue,
            internal::YGUnit::YGUnitPoint => StyleUnit::Point(OrderedFloat(v.value)),
            internal::YGUnit::YGUnitPercent => StyleUnit::Percent(OrderedFloat(v.value)),
            internal::YGUnit::YGUnitAuto => StyleUnit::Auto,
        }
    }
}
