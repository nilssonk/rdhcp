use crate::dhcp_packet::DhcpOptionView;
pub use core::borrow::Borrow;
pub use core::iter::Iterator;
pub use core::slice::IterMut;
use std::borrow::BorrowMut;

#[derive(Debug, PartialEq, Eq)]
pub struct DhcpOptionHeaderViewIterator<T> {
    data: T,
    offset: usize,
}

impl<T> DhcpOptionHeaderViewIterator<T> {
    pub(crate) fn from(data: T) -> Self {
        Self { data, offset: 0 }
    }
}

impl<T> Iterator for DhcpOptionHeaderViewIterator<T>
where
    T: Borrow<[u8]>,
{
    type Item = DhcpOptionView<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let data = self.data.borrow();
        if self.offset + 2 > data.len() {
            return None;
        }

        let result = DhcpOptionHeaderView::try_from(&data[self.offset..]);

        if let Some(opt) = &result {
            self.offset += opt.get_length();
        }

        result
    }
}

impl<T> IterMut for DhcpOptionHeaderViewIterator<T> where T: BorrowMut<[u8]> {}
