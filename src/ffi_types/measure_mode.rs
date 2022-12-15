use crate::internal;

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
