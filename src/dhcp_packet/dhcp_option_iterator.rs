use crate::DhcpOption;
pub use core::borrow::Borrow;
pub use core::iter::Iterator;

#[derive(Debug, PartialEq, Eq)]
pub struct DhcpOptionIterator<'a> {
    data: &'a [u8],
}

impl<'a> DhcpOptionIterator<'a> {
    pub(crate) fn from(data: &'a [u8]) -> Self {
        Self { data }
    }
}

impl<'a> Iterator for DhcpOptionIterator<'a> {
    type Item = DhcpOption<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }

        let result = DhcpOption::try_decode(self.data);

        if let Some(opt) = &result {
            self.data = &self.data[opt.get_length()..];
        }

        result
    }
}
