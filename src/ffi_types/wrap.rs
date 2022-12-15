use crate::internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum Wrap {
    NoWrap = 0,
    Wrap = 1,
    WrapReverse = 2,
}

impl From<Wrap> for internal::YGWrap {
    fn from(w: Wrap) -> internal::YGWrap {
        match w {
            Wrap::NoWrap => internal::YGWrap::YGWrapNoWrap,
            Wrap::Wrap => internal::YGWrap::YGWrapWrap,
            Wrap::WrapReverse => internal::YGWrap::YGWrapWrapReverse,
        }
    }
}

impl From<internal::YGWrap> for Wrap {
    fn from(w: internal::YGWrap) -> Wrap {
        match w {
            internal::YGWrap::YGWrapNoWrap => Wrap::NoWrap,
            internal::YGWrap::YGWrapWrap => Wrap::Wrap,
            internal::YGWrap::YGWrapWrapReverse => Wrap::WrapReverse,
        }
    }
}
