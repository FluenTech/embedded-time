use core::{cmp, ops};

#[derive(Copy, Clone, Debug)]
pub struct Ratio<T>
where
    T: Copy,
{
    numerator: T,
    denominator: T,
}

impl<T> Ratio<T>
where
    T: Copy + ops::Div<Output = T>,
{
    pub const fn new(numerator: T, denominator: T) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
}

impl<T: cmp::PartialEq + Copy> cmp::PartialEq for Ratio<T> {
    fn eq(&self, other: &Self) -> bool {
        self.numerator == other.numerator && self.denominator == other.denominator
    }
}

impl ops::Mul<Ratio<i64>> for i64 {
    type Output = i64;

    fn mul(self, rhs: Ratio<i64>) -> Self::Output {
        self * rhs.numerator / rhs.denominator
    }
}
