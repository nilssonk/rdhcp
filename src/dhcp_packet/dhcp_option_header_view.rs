use crate::dhcp_packet::DhcpOptionType;
use core::borrow::{Borrow, BorrowMut};
use core::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq, Eq)]
pub struct DhcpOptionHeaderView<T> {
    data: T,
}

impl<T> DhcpOptionHeaderView<T> {
    pub(crate) fn from(data: T) -> Self {
        Self { data }
    }
}

impl<T> DhcpOptionHeaderView<T>
where
    T: Borrow<[u8]>,
{
    #[inline(always)]
    fn padding_length(&self) -> usize {
        let data = self.data.borrow();
        let mut length = 1;
        while length < data.len() && data[length] == 0 {
            length += 1;
        }

        length
    }

    pub fn get_length(&self) -> usize {
        use DhcpOptionType::*;
        match self.deref() {
            Padding => self.padding_length(),
            MagicCookie => 4,
            End => 1,
            _ => self.data.borrow()[1] as usize,
        }
    }
}

impl<T> Deref for DhcpOptionHeaderView<T>
where
    T: Borrow<[u8]>,
{
    type Target = DhcpOptionType;
    fn deref(&self) -> &Self::Target {
        let tag = self.data.borrow()[0];

        use DhcpOptionType::*;
        match tag {
            0 => Some(Self::consolidated_padding(data)),
            99 => Some(Self {
                data,
                option_type: MagicCookie,
            }),
            255 => Some(End(&slice[..1])),
            _ => Self::try_decode_tlv(data),
        }
    }
}

impl<T> DerefMut for DhcpOptionHeaderView<T> where T: BorrowMut<[u8]> {}
