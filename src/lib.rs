#![cfg_attr(not(test), no_std)]

mod duration;
mod numerical_traits;

pub use duration::Duration;
pub use numerical_traits::NumericalDuration;

pub mod instant_trait {
    pub trait Instant: Copy {
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
///
/// The prelude may grow in minor releases. Any removals will only occur in
/// major releases.
pub mod prelude {
    // Rename traits to `_` to avoid any potential name conflicts.
    pub use crate::instant_trait::Instant as _Instant;
    pub use crate::NumericalDuration as _NumericalDuration;
}

#[cfg(test)]
mod tests {
    use super::{prelude::*, *};
    use core::fmt::{self, Display, Formatter};

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
