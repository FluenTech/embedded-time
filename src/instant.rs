use crate::duration::{Period, Time};
use crate::numerical_traits::NumericalDuration;
use crate::IntTrait;
use crate::Ratio;
use core::marker::PhantomData;
use core::{fmt, ops};

pub trait Clock {
    /// The type of the internal representation of time
    type Rep: IntTrait + NumericalDuration;
    const PERIOD: Period;

    /// Get the current Instant
    fn now<U: Time<Self::Rep>>() -> Instant<U>
    where
        Self: Sized;
}

#[derive(Debug, Copy, Clone)]
pub struct Instant<T>(pub T);

impl<T> Instant<T> {
    pub fn duration_since_epoch(self) -> T {
        self.0
    }
}

// impl<C: Clock> ops::Add<Duration<C::Rep>> for Instant<C>
// where
//     C::Rep: ops::Add,
//     Duration<C::Rep>: ops::Add<Output = Duration<C::Rep>>,
// {
//     type Output = Self;
//
//     fn add(self, rhs: Duration<C::Rep>) -> Self::Output {
//         Self(self.0 + rhs)
//     }
// }
//
// impl<C: Clock> ops::Sub for Instant<C> {
//     type Output = Duration<C::Rep>;
//
//     fn sub(self, rhs: Self) -> Self::Output {
//         self.0 - rhs.0
//     }
// }
//
// impl<C: Clock> ops::Sub<Duration<C::Rep>> for Instant<C> {
//     type Output = Self;
//
//     fn sub(self, rhs: Duration<C::Rep>) -> Self::Output {
//         Self(self.0 - rhs)
//     }
// }

// impl<C: Clock> fmt::Display for Instant<C>
// where
//     Duration<C::Rep>: fmt::Display,
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         self.0.fmt(f)
//     }
// }
