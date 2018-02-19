use internal;
pub use internal::YGMeasureMode as YGInternalMeasureMode;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum MeasureMode {
	Undefined = 0,
	Exactly = 1,
	AtMost = 2,
}

impl From<MeasureMode> for internal::YGMeasureMode {
	fn from(m: MeasureMode) -> internal::YGMeasureMode {
		match m {
			MeasureMode::Undefined => internal::YGMeasureMode::YGMeasureModeUndefined,
			MeasureMode::Exactly => internal::YGMeasureMode::YGMeasureModeExactly,
			MeasureMode::AtMost => internal::YGMeasureMode::YGMeasureModeAtMost,
		}
	}
}

impl From<internal::YGMeasureMode> for MeasureMode {
	fn from(m: internal::YGMeasureMode) -> MeasureMode {
		match m {
			internal::YGMeasureMode::YGMeasureModeUndefined => MeasureMode::Undefined,
			internal::YGMeasureMode::YGMeasureModeExactly => MeasureMode::Exactly,
			internal::YGMeasureMode::YGMeasureModeAtMost => MeasureMode::AtMost,
		}
	}
}
