use core::borrow::{Borrow, BorrowMut};
use std::net::Ipv4Addr;

#[derive(Debug, PartialEq, Eq)]
pub struct Ipv4AddrView<T> {
    data: T,
}

impl<T> Ipv4AddrView<T>
where
    T: Borrow<[u8]>,
{
    #[allow(unused)] // Remove upon first use
    pub(crate) fn from(data: T) -> Self {
        Self { data }
    }

    pub fn to_owned(&self) -> Ipv4Addr {
        let data: [u8; 4] = self.data.borrow()[..4].try_into().unwrap();
        data.into()
    }
}

impl<T> Ipv4AddrView<T>
where
    T: BorrowMut<[u8]>,
{
    #[allow(unused)] // Remove upon first use
    pub(crate) fn from_mut(data: T) -> Self {
        Self { data }
    }
}

#[allow(unused)] // Remove upon first use
macro_rules! ipv4_addr_view{
    {} =>
    {
        Ipv4AddrView<impl Borrow<[u8]> + '_>
    }
}

#[allow(unused)] // Remove upon first use
macro_rules! ipv4_addr_view_mut{
    {} =>
    {
        Ipv4AddrView<impl BorrowMut<[u8]> + '_>
    }
}
