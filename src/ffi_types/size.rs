use crate::internal;

#[repr(C)]
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl From<Size> for internal::YGSize {
    fn from(s: Size) -> internal::YGSize {
        internal::YGSize { width: s.width, height: s.height }
    }
}
