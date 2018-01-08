use internal;

#[repr(C)]
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Size {
	pub width: f32,
	pub height: f32,
}

impl From<Size> for internal::YGSize {
	fn from(s: Size) -> internal::YGSize {
		internal::YGSize {
			width: s.width,
			height: s.height,
		}
	}
}
