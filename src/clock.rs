use crate::instant::Instant;
use crate::numerical_duration::TimeRep;
use crate::Period;
use core::fmt;

pub trait Clock: Sized + Period {
    /// The type of the internal representation of time
    type Rep: TimeRep;

    /// Get the current Instant
    fn now() -> Instant<Self>;
}
