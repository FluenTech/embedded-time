#![cfg_attr(not(test), no_std)]
#![feature(const_fn)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

mod duration;
mod numerical_traits;
mod ratio;

pub use duration::Duration;
pub use ratio::Ratio;

pub mod instant_trait {
    use core::ops;

    pub trait Instant:
        Copy
        + ops::Add<crate::Duration>
        + ops::Sub<Output = crate::Duration>
        + ops::Sub<crate::Duration>
    {
        fn now() -> Self;

        fn elapsed(self) -> crate::Duration;

        fn duration_since_epoch(self) -> crate::Duration;
    }
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
    pub use crate::instant_trait::Instant as _;
    pub use crate::numerical_traits::NumericalDuration as _;
}

#[cfg(test)]
mod tests {
    use super::{prelude::*, *};
    use core::fmt::{self, Display, Formatter};
    use core::ops;

    #[derive(Debug, Copy, Clone)]
    struct Instant(Duration);

    impl Instant {}

    impl instant_trait::Instant for Instant {
        fn now() -> Self {
            Self(5_025_678_910_111_i64.nanoseconds())
        }

        fn elapsed(self) -> Duration {
            unimplemented!()
        }

        fn duration_since_epoch(self) -> Duration {
            self.0
        }
    }

    impl ops::Add<Duration> for Instant {
        type Output = Duration;

        fn add(self, rhs: Duration) -> Self::Output {
            self.0 + rhs
        }
    }

    impl ops::Sub for Instant {
        type Output = Duration;

        fn sub(self, rhs: Self) -> Self::Output {
            self.0 - rhs.0
        }
    }

    impl ops::Sub<Duration> for Instant {
        type Output = Self;

        fn sub(self, rhs: Duration) -> Self::Output {
            Self(self.0 - rhs)
        }
    }

    impl Display for Instant {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
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
