//! This crate provides a way to abstract over hardware-specific timing providers such as timers or counters
//! In addition it provides comprehensive `Instant` and duration types (`Minutes`, `Seconds`, `Milliseconds`, etc.) along
//! with intuitive interfaces.
//!
//! # Example Usage
//! ```rust
//! use embedded_time::prelude::*;
//! use embedded_time::time_units::*;
//!
//!
//! ```

#![cfg_attr(not(test), no_std)]
#![feature(associated_type_bounds)]
#![warn(missing_doc_code_examples)]
// #![warn(clippy::pedantic)]

pub mod duration;
mod instant;
mod integer;
pub mod numerical_duration;

pub use duration::time_units;
pub use duration::Duration;
pub use instant::{Clock, Instant};

pub use num::rational::Ratio;

pub trait Period {
    const PERIOD: Ratio<i32>;
}

pub trait Wrapper: Sized {
    type Rep;

    fn unwrap(self) -> Self::Rep;
}

/// A collection of imports that are widely useful.
///
/// Unlike the standard library, this must be explicitly imported:
///
/// ```rust,no_run
/// use embedded_time::prelude::*;
/// ```
/// The prelude may grow in minor releases. Any removals will only occur in
/// major releases.
pub mod prelude {
    // Rename traits to `_` to avoid any potential name conflicts.
    pub use crate::duration::time_units::TryConvertFrom as _TryConvertFrom;
    pub use crate::duration::time_units::TryConvertInto as _TryConvertInto;
    pub use crate::duration::Duration as _Duration;
    pub use crate::integer::IntTrait as _IntTrait;
    pub use crate::numerical_duration::TimeRep as _TimeRep;
    pub use crate::Period as _Period;
    pub use num::Integer as _Integer;
}

#[cfg(test)]
mod tests {
    use crate::numerical_duration::TimeRep;
    use crate::prelude::*;
    use crate::time_units::*;
    use crate::Ratio;
    use crate::{Clock, Duration, Instant, Period};

    struct MockClock64;
    impl Clock for MockClock64 {
        type Rep = i64;

        fn now<Dur>() -> Instant<Dur>
        where
            Dur: Duration,
            Dur::Rep: TimeRep,
        {
            Instant(Dur::from_ticks(48_000_i64, MockClock64::PERIOD))
        }
    }
    impl Period for MockClock64 {
        const PERIOD: Ratio<i32> = Ratio::new_raw(1, 16_000_000);
    }

    struct MockClock32;
    impl Clock for MockClock32 {
        type Rep = i32;

        fn now<Dur>() -> Instant<Dur>
        where
            Dur: Duration,
            Dur::Rep: TimeRep,
        {
            Instant(Dur::from_ticks(192_000_i32, MockClock32::PERIOD))
        }
    }
    impl Period for MockClock32 {
        const PERIOD: Ratio<i32> = Ratio::new_raw(1, 64_000_000);
    }

    fn get_time<M>()
    where
        M: Clock,
        M::Rep: TimeRep,
    {
        assert_eq!(M::now::<Milliseconds<i32>>(), Instant(Milliseconds(3_i64)));
    }

    #[test]
    fn common_types() {
        let then = MockClock32::now::<Nanoseconds<i64>>();
        let now = MockClock64::now::<Milliseconds<i32>>();

        get_time::<MockClock64>();
        get_time::<MockClock32>();

        let then = then - Seconds(1);
        assert_ne!(then, now);
        assert!(then < now);
    }

    #[test]
    fn brute_force() {
        let mut time = 1_i64;
        time *= 60;
        assert_eq!(Hours(1), Minutes(time));
        time *= 60;
        assert_eq!(Hours(1), Seconds(time));
        time *= 1000;
        assert_eq!(Hours(1), Milliseconds(time));
        time *= 1000;
        assert_eq!(Hours(1), Microseconds(time));
        time *= 1000;
        assert_eq!(Hours(1), Nanoseconds(time));
    }
}
