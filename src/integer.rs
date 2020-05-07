use core::{fmt, ops};

pub trait IntTrait: num::Integer + num::PrimInt + From<i32> + fmt::Display {}

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
