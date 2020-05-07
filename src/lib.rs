#![cfg_attr(not(test), no_std)]
#![feature(const_fn)]
#![feature(const_trait_impl)]
#![feature(const_generics)]
#![feature(associated_type_bounds)]
#![feature(type_alias_impl_trait)]
#![feature(associated_type_defaults)]
#![allow(incomplete_features)]
#![warn(missing_doc_code_examples)]

pub mod duration;
mod instant;
mod integer;
mod numerical_duration;

pub use instant::Clock;
pub use instant::Instant;
pub use integer::{IntTrait, Integer};
pub use num::rational::Ratio;

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
    pub use crate::duration::Time as _Time;
    pub use crate::integer::IntTrait as _IntTrait;
    pub use crate::numerical_duration::NumericalDuration as _NumericalDuration;
    pub use num::Integer as _Integer;
}

#[cfg(test)]
mod tests {
    use super::{prelude::*, *};
    use crate::duration::{Milliseconds, Period, Seconds, Time};
    use crate::instant::{Clock, Instant};

    #[derive(Copy, Clone)]
    struct MockClock;

    impl Clock for MockClock {
        type Rep = i64;
        const PERIOD: Period = Period::new_raw(1, 1_000);

        fn now<U: Time<Self::Rep>>() -> Instant<U>
        where
            Self: Sized,
        {
            Instant(U::new(5_025_678_910_111))
        }
    }

    #[test]
    fn it_works() {
        let now = MockClock::now::<Milliseconds<_>>();
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
        // assert_eq!(format!("{}", now), "01:23:45.678");
        // assert_eq!(format!("{}", now.duration_since_epoch()), "01:23:45.678");
    }
}
