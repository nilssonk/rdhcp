use crate::DhcpOption;
pub use core::borrow::Borrow;
pub use core::iter::Iterator;

#[allow(dead_code)] // Remove upon first use
#[derive(Debug)]
pub struct DhcpOptionIterator<T> {
    data: T,
    offset: usize,
}

impl<T> DhcpOptionIterator<T> {
    pub(crate) fn from(data: T) -> Self {
        const OFFSET: usize = 236;
        Self {
            data,
            offset: OFFSET,
        }
    }
}

impl<T> Iterator for DhcpOptionIterator<T>
where
    T: Borrow<[u8]>,
{
    type Item = DhcpOption;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
