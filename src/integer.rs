use core::convert::TryFrom;
use core::convert::TryInto;
use core::{fmt, ops};

pub trait IntTrait:
    From<i32>
    + TryFrom<Self, Error: fmt::Debug>
    + TryFrom<i32, Error: fmt::Debug>
    + num::PrimInt
    + TryInto<i32, Error: fmt::Debug>
    + fmt::Display
    + fmt::Debug
    + Into<i64>
    + TryInto<i64, Error: fmt::Debug>
    + TryFrom<i64, Error: fmt::Debug>
{
}

impl IntTrait for i32 {}
impl IntTrait for i64 {}

#[derive(Copy, Clone, Debug, Default)]
pub struct Integer<T: IntTrait>(pub T);

impl<T: IntTrait> ops::Deref for Integer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
