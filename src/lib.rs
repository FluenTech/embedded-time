#![cfg_attr(not(test), no_std)]
#![feature(const_fn)]
#![feature(const_trait_impl)]
#![feature(const_generics)]
#![feature(associated_type_bounds)]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]

mod duration;
mod instant;
mod numerical_traits;
mod ratio;

pub use duration::Duration;
pub use instant::Clock;
pub use instant::Instant;
pub use ratio::IntTrait;
pub use ratio::Ratio;

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
    pub use crate::duration::AbsSigned as _AbsSigned;
    pub use crate::duration::AbsUnsigned as _AbsUnsigned;
    pub use crate::numerical_traits::NumericalDuration as _NumericalDuration;
    pub use crate::ratio::IntTrait as _IntTrait;
    pub use num_traits::PrimInt as _PrimInt;
    pub use num_traits::Signed as _Signed;
    pub use num_traits::Unsigned as _Unsigned;
}

#[cfg(test)]
mod tests {
    use super::{prelude::*, *};
    use core::fmt::{self, Display, Formatter};
    use core::ops;

    struct MockClock;

    impl Clock for MockClock {
        type Rep = i64;
        const PERIOD: Ratio<Self::Rep> = Ratio::<Self::Rep>::new(1, 1_000);

        fn now() -> Instant<Self>
        where
            Self: Sized,
        {
            Instant::<Self>(Duration::<Self::Rep>::new(5_025_678_910_111, Self::Period))
        }
    }

    #[test]
    fn it_works() {
        let now = Instant::now();
        assert_eq!(
            now.duration_since_epoch(),
            5_025_678_910_111_i64.nanoseconds()
        );
        assert_eq!(now.duration_since_epoch().as_micros(), 5_025_678_910);
        assert_eq!(now.duration_since_epoch().as_millis(), 5_025_678);
        assert_eq!(format!("{}", now), "01:23:45.678");
        assert_eq!(format!("{}", now.duration_since_epoch()), "01:23:45.678");
    }
}
