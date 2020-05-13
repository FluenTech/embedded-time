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
    pub use crate::duration::Duration as _Duration;
    pub use crate::integer::IntTrait as _IntTrait;
    pub use crate::numerical_duration::TimeRep as _TimeRep;
    pub use num::Integer as _Integer;
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::time_units::*;
    use crate::Ratio;
    use crate::{Clock, Duration, Instant, Period};

    struct MockClock;

    impl Clock for MockClock {
        type Rep = i64;

        fn now<Dur>() -> Instant<Dur>
        where
            Dur: Duration<Self::Rep>,
        {
            Instant(Dur::new(5_025_678_910_111))
        }
    }

    impl Period for MockClock {
        const PERIOD: Ratio<i32> = Ratio::<i32>::new_raw(1, 1_000);
    }

    #[test]
    fn common_types() {
        let then = Instant(Milliseconds::<i64>(5_025_678_910_110));
        let now = MockClock::now::<Milliseconds<_>>();
        let _now_seconds = MockClock::now::<Seconds<_>>();
        // let now32 = i32::try_convert_from(MockClock::now::<Milliseconds<_>>()).unwrap();

        assert_ne!(then, now);
        assert!(then < now);
    }

    #[test]
    fn brute_force() {
        let mut time = 1_i64;
        time *= 60;
        assert_eq!(Hours(1_i64), Minutes(time));
        time *= 60;
        assert_eq!(Hours(1_i64), Seconds(time));
        time *= 1000;
        assert_eq!(Hours(1_i64), Milliseconds(time));
        time *= 1000;
        assert_eq!(Hours(1_i64), Microseconds(time));
        time *= 1000;
        assert_eq!(Hours(1_i64), Nanoseconds(time));
    }
}
