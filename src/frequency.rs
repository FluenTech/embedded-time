//! Representations of frequency-based values

pub(crate) mod units {
    use crate::{Period, TimeInt};
    use core::{convert, ops};

    /// A frequency unit type
    #[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
    pub struct Hertz<T: TimeInt = u32>(pub T);

    impl<T: TimeInt> Hertz<T> {
        /// ```rust
        /// # use embedded_time::{Period, units::*};
        /// assert_eq!(Hertz(1_000_u32).into_period(), <Period>::new(1, 1_000));
        /// ```
        pub fn into_period(self) -> Period<T> {
            Period::from_frequency(self)
        }

        /// ```rust
        /// # use embedded_time::{Period, units::*};
        /// assert_eq!(Hertz::from_period(Period::new(1, 1_000)), Hertz(1_000_u32));
        /// ```
        pub fn from_period(period: Period<T>) -> Self {
            period.to_frequency()
        }
    }

    impl<T: TimeInt, Rhs: TimeInt> ops::Mul<Rhs> for Hertz<T>
    where
        T: From<Rhs>,
    {
        type Output = Self;

        /// ```rust
        /// # use embedded_time::units::*;
        /// assert_eq!(Hertz(100_u32) * 3_u32, Hertz(300_u32));
        /// ```
        fn mul(self, rhs: Rhs) -> Self::Output {
            Self(self.0 * <T as convert::From<Rhs>>::from(rhs))
        }
    }

    impl<T: TimeInt, Rhs: TimeInt> ops::Div<Rhs> for Hertz<T>
    where
        T: From<Rhs>,
    {
        type Output = Self;

        /// ```rust
        /// # use embedded_time::units::*;
        /// assert_eq!(Hertz(300_u32) / 3_u32, Hertz(100_u32));
        /// ```
        fn div(self, rhs: Rhs) -> Self::Output {
            Self(self.0 / <T as convert::From<Rhs>>::from(rhs))
        }
    }
}
