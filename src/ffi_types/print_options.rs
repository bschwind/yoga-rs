use internal;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PrintOptions {
	Layout = 1,
	Style = 2,
	Children = 4,
}

impl From<PrintOptions> for internal::YGPrintOptions {
	fn from(p: PrintOptions) -> internal::YGPrintOptions {
		match p {
			PrintOptions::Layout => internal::YGPrintOptions::YGPrintOptionsLayout,
			PrintOptions::Style => internal::YGPrintOptions::YGPrintOptionsStyle,
			PrintOptions::Children => internal::YGPrintOptions::YGPrintOptionsChildren,
		}
	}
}
