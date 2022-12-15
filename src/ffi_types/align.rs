use crate::internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum Align {
    Auto = 0,
    FlexStart = 1,
    Center = 2,
    FlexEnd = 3,
    Stretch = 4,
    Baseline = 5,
    SpaceBetween = 6,
    SpaceAround = 7,
}

impl From<Align> for internal::YGAlign {
    fn from(a: Align) -> internal::YGAlign {
        match a {
            Align::Auto => internal::YGAlign::YGAlignAuto,
            Align::FlexStart => internal::YGAlign::YGAlignFlexStart,
            Align::Center => internal::YGAlign::YGAlignCenter,
            Align::FlexEnd => internal::YGAlign::YGAlignFlexEnd,
            Align::Stretch => internal::YGAlign::YGAlignStretch,
            Align::Baseline => internal::YGAlign::YGAlignBaseline,
            Align::SpaceBetween => internal::YGAlign::YGAlignSpaceBetween,
            Align::SpaceAround => internal::YGAlign::YGAlignSpaceAround,
        }
    }
}

impl From<internal::YGAlign> for Align {
    fn from(a: internal::YGAlign) -> Align {
        match a {
            internal::YGAlign::YGAlignAuto => Align::Auto,
            internal::YGAlign::YGAlignFlexStart => Align::FlexStart,
            internal::YGAlign::YGAlignCenter => Align::Center,
            internal::YGAlign::YGAlignFlexEnd => Align::FlexEnd,
            internal::YGAlign::YGAlignStretch => Align::Stretch,
            internal::YGAlign::YGAlignBaseline => Align::Baseline,
            internal::YGAlign::YGAlignSpaceBetween => Align::SpaceBetween,
            internal::YGAlign::YGAlignSpaceAround => Align::SpaceAround,
        }
    }
}
