use core::borrow::{Borrow, BorrowMut};

#[derive(Debug, PartialEq, Eq)]
pub struct DhcpPacketFlagsView<T> {
    data: T,
}

impl<T> DhcpPacketFlagsView<T>
where
    T: Borrow<[u8]>,
{
    #[allow(unused)] // Remove upon first use
    pub(crate) fn from(data: T) -> Self {
        Self { data }
    }

    pub fn is_broadcast(&self) -> bool {
        let data = self.data.borrow();
        data[0] & 0x1 != 0
    }
}

impl<T> DhcpPacketFlagsView<T>
where
    T: BorrowMut<[u8]>,
{
    #[allow(unused)] // Remove upon first use
    pub(crate) fn from_mut(data: T) -> Self {
        Self { data }
    }
}
