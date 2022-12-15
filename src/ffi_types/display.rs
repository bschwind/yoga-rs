use crate::internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum Display {
    Flex = 0,
    None = 1,
}

impl From<Display> for internal::YGDisplay {
    fn from(d: Display) -> internal::YGDisplay {
        match d {
            Display::Flex => internal::YGDisplay::YGDisplayFlex,
            Display::None => internal::YGDisplay::YGDisplayNone,
        }
    }
}
