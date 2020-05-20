//! # Embedded Time
//! `embedded-time` provides a way (using the [`Clock`](trait.Clock.html) trait) to abstract over
//! hardware-specific timing providers such as peripheral timers
//! In addition it provides comprehensive [`Instant`](instant::Instant) and duration types
//! ([`Minutes`](time_units::Minutes), [`Seconds`](time_units::Seconds),
//! [`Milliseconds`](time_units::Milliseconds), etc.) along with intuitive interfaces.
//!
//! # Example Usage
//! ```rust,no_run
//! # use embedded_time::prelude::*;
//! # use embedded_time::time_units::*;
//! # use embedded_time::{Ratio, Duration, TimeRep};
//! # use embedded_time::instant::Instant;
//! # #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
//! # struct SomeClock;
//! # impl embedded_time::Clock for SomeClock {
//! #     type Rep = i64;
//! #     fn now() -> Instant<Self> { unimplemented!() } }
//! # impl embedded_time::Period for SomeClock { const PERIOD: Ratio<i32> = Ratio::<i32>::new_raw(1, 16_000_000); }
//! #
//! let instant1 = SomeClock::now();
//! // ...
//! let instant2 = SomeClock::now();
//! assert!(instant1 < instant2);    // instant1 is *before* instant2
//!
//! let duration: Option<Microseconds<i64>> = instant2.elapsed_since(instant1);    // duration is the difference between the instances
//! assert!(duration.is_some());
//! assert_eq!(instant1 + duration.unwrap(), instant2);
//! ```

#![cfg_attr(not(test), no_std)]
#![feature(associated_type_bounds)]
#![feature(type_alias_impl_trait)]
#![deny(intra_doc_link_resolution_failure)]
// #![warn(clippy::pedantic)]

mod clock;
pub mod duration;
pub mod instant;
mod numerical_duration;

pub use clock::Clock;
pub use duration::{time_units, Duration};
pub use num::rational::Ratio;
pub use numerical_duration::TimeRep;

pub trait Period {
    const PERIOD: Ratio<i32>;
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
    pub use crate::duration::Duration as _;
    pub use crate::duration::TryConvertFrom as _;
    pub use crate::duration::TryConvertInto as _;
    pub use crate::numerical_duration::TimeRep as _;
    pub use crate::Clock as _;
    pub use crate::Period as _;
    pub use num::Integer as _;
}

#[cfg(test)]
mod tests {
    use crate::instant::Instant;
    use crate::prelude::*;
    use crate::time_units::*;
    use crate::Ratio;
    use crate::{Clock, Period};

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct MockClock64;
    impl Clock for MockClock64 {
        type Rep = i64;

        fn now() -> Instant<Self> {
            Instant::new(128_000_000)
        }
    }
    impl Period for MockClock64 {
        const PERIOD: Ratio<i32> = Ratio::new_raw(1, 16_000_000);
    }

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct MockClock32;
    impl Clock for MockClock32 {
        type Rep = i32;

        fn now() -> Instant<Self> {
            Instant::new(32_000_000)
        }
    }
    impl Period for MockClock32 {
        const PERIOD: Ratio<i32> = Ratio::new_raw(1, 64_000_000);
    }

    fn get_time<M>()
    where
        M: Clock,
    {
        assert_eq!(M::now().elapsed_since_epoch(), Some(Seconds(2)));
    }

    #[test]
    fn common_types() {
        let then = MockClock32::now();
        let now = MockClock32::now();

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
