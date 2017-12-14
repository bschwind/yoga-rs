use internal;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct Size {
	pub width: f32,
	pub height: f32,
}

impl Clone for Size {
	fn clone(&self) -> Self {
		*self
	}
}

impl From<Size> for internal::YGSize {
	fn from(s: Size) -> internal::YGSize {
		internal::YGSize {
			width: s.width,
			height: s.height,
		}
	}
}
