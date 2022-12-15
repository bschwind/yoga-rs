use crate::types::StyleUnit;
use ordered_float::OrderedFloat;

pub trait Percent {
    fn percent(self) -> StyleUnit;
}

impl Percent for f32 {
    fn percent(self) -> StyleUnit {
        StyleUnit::Percent(OrderedFloat(self))
    }
}

impl Percent for i32 {
    fn percent(self) -> StyleUnit {
        StyleUnit::Percent(OrderedFloat(self as f32))
    }
}

pub trait Point {
    fn point(self) -> StyleUnit;
}

impl Point for f32 {
    fn point(self) -> StyleUnit {
        StyleUnit::Point(OrderedFloat(self))
    }
}

impl Point for i32 {
    fn point(self) -> StyleUnit {
        StyleUnit::Point(OrderedFloat(self as f32))
    }
}
