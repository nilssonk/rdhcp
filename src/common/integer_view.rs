use core::borrow::{Borrow, BorrowMut};
use core::marker::PhantomData;

use crate::common::BeInteger;

#[derive(Debug, PartialEq, Eq)]
pub struct IntegerView<T, U> {
    data: U,
    phantom: PhantomData<T>,
}

impl<T, U> IntegerView<T, U>
where
    U: Borrow<[u8]>,
{
    pub(crate) fn from(data: U) -> Self {
        Self {
            data,
            phantom: PhantomData,
        }
    }
}

impl<T, U> IntegerView<T, U>
where
    T: BeInteger,
    U: Borrow<[u8]>,
{
    pub fn get(&self) -> T {
        <T as BeInteger>::read_be(self.data.borrow())
    }
}

impl<T, U> IntegerView<T, U>
where
    T: BeInteger,
    U: BorrowMut<[u8]>,
{
    #[allow(unused)] // Remove upon first use
    pub(crate) fn from_mut(data: U) -> Self {
        Self {
            data,
            phantom: PhantomData,
        }
    }

    pub fn set(&mut self, value: T) {
        value.write_be(self.data.borrow_mut());
    }
}

macro_rules! integer_view{
    {$t:ty} =>
    {
        IntegerView<$t, impl Borrow<[u8]> + '_>
    }
}
pub(crate) use integer_view;

macro_rules! integer_view_mut{
    {$t:ty} =>
    {
        IntegerView<$t, impl BorrowMut<[u8]> + '_>
    }
}
pub(crate) use integer_view_mut;
