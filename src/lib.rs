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
#![feature(const_fn)]
#![feature(associated_type_bounds)]
#![feature(type_alias_impl_trait)]
#![warn(missing_doc_code_examples)]
// #![warn(clippy::pedantic)]

pub mod duration;
mod instant;
mod integer;
pub mod numerical_duration;

pub use duration::time_units;
pub use duration::Duration;
pub use instant::{Clock, Instant};

use num::rational::Ratio;

pub type Period = Ratio<i32>;

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
    pub use crate::duration::Duration as _Duration;
    pub use crate::integer::IntTrait as _IntTrait;
    pub use crate::numerical_duration::TimeRep as _TimeRep;
    pub use num::Integer as _Integer;
}

#[cfg(test)]
mod tests {
    use crate::numerical_duration::TimeRep;
    use crate::prelude::*;
    use crate::time_units::*;
    use crate::{Clock, Duration, Instant, Period};

    struct MockClock;

    impl Clock for MockClock {
        type Rep = i64;
        const PERIOD: Period = Period::new_raw(1, 1_000);

        fn now<U>() -> Instant<U, Self::Rep>
        where
            Self: Sized,
            U: Duration<Self::Rep>,
        {
            Instant::new(U::new(5_025_678_910_111))
        }
    }

    #[test]
    fn common_types() {
        let then = Instant::new(Milliseconds::<i64>(5_025_678_910_110));
        let now = MockClock::now::<Milliseconds<_>>();

        assert_ne!(then, now);
        assert!(then < now);
        // assert_eq!(
        //     now.duration_since_epoch(),
        //     5_025_678_910_111_i64.nanoseconds()
        // );
        // assert_eq!(now.duration_since_epoch().as_micros(), 5_025_678_910);
        assert_eq!(now.duration_since_epoch().count(), 5_025_678_910_111);
        assert_eq!(
            Seconds::from_dur(now.duration_since_epoch()).count(),
            5_025_678_910
        );
        assert_eq!(format!("{}", now.duration_since_epoch()), "5025678910111");
        assert_eq!(format!("{}", now), "5025678910111");
    }
}
