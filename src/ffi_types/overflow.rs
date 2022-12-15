use crate::internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum Overflow {
    Visible = 0,
    Hidden = 1,
    Scroll = 2,
}

impl From<Overflow> for internal::YGOverflow {
    fn from(o: Overflow) -> internal::YGOverflow {
        match o {
            Overflow::Visible => internal::YGOverflow::YGOverflowVisible,
            Overflow::Hidden => internal::YGOverflow::YGOverflowHidden,
            Overflow::Scroll => internal::YGOverflow::YGOverflowScroll,
        }
    }
}

impl From<internal::YGOverflow> for Overflow {
    fn from(o: internal::YGOverflow) -> Overflow {
        match o {
            internal::YGOverflow::YGOverflowVisible => Overflow::Visible,
            internal::YGOverflow::YGOverflowHidden => Overflow::Hidden,
            internal::YGOverflow::YGOverflowScroll => Overflow::Scroll,
        }
    }
}
