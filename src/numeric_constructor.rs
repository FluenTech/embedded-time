//! Construction of time-based types from integers

use crate::{duration::units::*, rate::units::*, time_int::TimeInt};

/// Create time-based values from primitive and core numeric types.
///
/// This trait is anonomously re-exported in [`traits`](crate::traits)
///
/// # Examples
/// Basic construction of time-based values.
/// ```rust
/// # use embedded_time::{traits::*, duration::units::*, rate::units::*};
/// assert_eq!(5_u32.nanoseconds(), Nanoseconds(5_u32));
/// assert_eq!(5_u32.microseconds(), Microseconds(5_u32));
/// assert_eq!(5_u32.milliseconds(), Milliseconds(5_u32));
/// assert_eq!(5_u32.seconds(), Seconds(5_u32));
/// assert_eq!(5_u32.minutes(), Minutes(5_u32));
/// assert_eq!(5_u32.hours(), Hours(5_u32));
///
/// assert_eq!(5_u32.MHz(), Megahertz(5_u32));
/// assert_eq!(5_u32.kHz(), Kilohertz(5_u32));
/// assert_eq!(5_u32.Hz(), Hertz(5_u32));
/// assert_eq!(5_u32.MiBps(), MebibytesPerSecond(5_u32));
/// assert_eq!(5_u32.MBps(), MegabytesPerSecond(5_u32));
/// assert_eq!(5_u32.KiBps(), KibibytesPerSecond(5_u32));
/// assert_eq!(5_u32.kBps(), KiloBytesPerSecond(5_u32));
/// assert_eq!(5_u32.Bps(), BytesPerSecond(5_u32));
/// assert_eq!(5_u32.Mibps(), MebibitsPerSecond(5_u32));
/// assert_eq!(5_u32.Mbps(), MegabitsPerSecond(5_u32));
/// assert_eq!(5_u32.Kibps(), KibibitsPerSecond(5_u32));
/// assert_eq!(5_u32.kbps(), KilobitsPerSecond(5_u32));
/// assert_eq!(5_u32.bps(), BitsPerSecond(5_u32));
/// assert_eq!(5_u32.MBd(), Megabaud(5_u32));
/// assert_eq!(5_u32.kBd(), Kilobaud(5_u32));
/// assert_eq!(5_u32.Bd(), Baud(5_u32));
/// ```
#[allow(non_snake_case)]
pub trait NumericConstructor: TimeInt {
    /// Construct the duration implementation
    fn nanoseconds(self) -> Nanoseconds<Self>;
    /// Construct the duration implementation
    fn microseconds(self) -> Microseconds<Self>;
    /// Construct the duration implementation
    fn milliseconds(self) -> Milliseconds<Self>;
    /// Construct the duration implementation
    fn seconds(self) -> Seconds<Self>;
    /// Construct the duration implementation
    fn minutes(self) -> Minutes<Self>;
    /// Construct the duration implementation
    fn hours(self) -> Hours<Self>;

    /// megahertz
    fn MHz(self) -> Megahertz<Self>;
    /// kilohertz
    fn kHz(self) -> Kilohertz<Self>;
    /// hertz
    fn Hz(self) -> Hertz<Self>;
    /// mebibytes per second
    fn MiBps(self) -> MebibytesPerSecond<Self>;
    /// megabytes per second
    fn MBps(self) -> MegabytesPerSecond<Self>;
    /// kibibytes per second
    fn KiBps(self) -> KibibytesPerSecond<Self>;
    /// kiloBytes per second
    fn kBps(self) -> KiloBytesPerSecond<Self>;
    /// bytes per second
    fn Bps(self) -> BytesPerSecond<Self>;
    /// mebibits per second
    fn Mibps(self) -> MebibitsPerSecond<Self>;
    /// megabits per second
    fn Mbps(self) -> MegabitsPerSecond<Self>;
    /// kibibits per second
    fn Kibps(self) -> KibibitsPerSecond<Self>;
    /// kilobits per second
    fn kbps(self) -> KilobitsPerSecond<Self>;
    /// bits per second
    fn bps(self) -> BitsPerSecond<Self>;
    /// megabaud
    fn MBd(self) -> Megabaud<Self>;
    /// kilobaud
    fn kBd(self) -> Kilobaud<Self>;
    /// baud
    fn Bd(self) -> Baud<Self>;
}

macro_rules! impl_numeric_constructors {
        ($($type:ty),* $(,)?) => {
            $(
                impl NumericConstructor for $type {
                    #[inline(always)]
                    fn nanoseconds(self) -> Nanoseconds<$type> {
                        Nanoseconds(self)
                    }

                    #[inline(always)]
                    fn microseconds(self) -> Microseconds<$type> {
                        Microseconds(self)
                    }

                    #[inline(always)]
                    fn milliseconds(self) -> Milliseconds<$type> {
                        Milliseconds(self)
                    }

                    #[inline(always)]
                    fn seconds(self) -> Seconds<$type> {
                        Seconds(self)
                    }

                    #[inline(always)]
                    fn minutes(self) -> Minutes<$type> {
                        Minutes(self)
                    }

                    #[inline(always)]
                    fn hours(self) -> Hours<$type> {
                        Hours(self)
                    }


                    fn MHz(self) -> Megahertz<$type> {
                        Megahertz(self)
                    }

                    fn kHz(self) -> Kilohertz<$type> {
                        Kilohertz(self)
                    }

                    fn Hz(self) -> Hertz<$type> {
                        Hertz(self)
                    }

                    fn MiBps(self) -> MebibytesPerSecond<$type> {
                        MebibytesPerSecond(self)
                    }

                    fn MBps(self) -> MegabytesPerSecond<$type> {
                        MegabytesPerSecond(self)
                    }

                    fn KiBps(self) -> KibibytesPerSecond<$type> {
                        KibibytesPerSecond(self)
                    }

                    fn kBps(self) -> KiloBytesPerSecond<$type> {
                        KiloBytesPerSecond(self)
                    }

                    fn Bps(self) -> BytesPerSecond<$type> {
                        BytesPerSecond(self)
                    }

                    fn Mibps(self) -> MebibitsPerSecond<$type> {
                        MebibitsPerSecond(self)
                    }

                    fn Mbps(self) -> MegabitsPerSecond<$type> {
                        MegabitsPerSecond(self)
                    }

                    fn Kibps(self) -> KibibitsPerSecond<$type> {
                        KibibitsPerSecond(self)
                    }

                    fn kbps(self) -> KilobitsPerSecond<$type> {
                        KilobitsPerSecond(self)
                    }

                    fn bps(self) -> BitsPerSecond<$type> {
                        BitsPerSecond(self)
                    }

                    fn MBd(self) -> Megabaud<$type> {
                        Megabaud(self)
                    }

                    fn kBd(self) -> Kilobaud<$type> {
                        Kilobaud(self)
                    }

                    fn Bd(self) -> Baud<$type> {
                        Baud(self)
                    }
                }
            )*
        };
    }

impl_numeric_constructors![u32, u64];
