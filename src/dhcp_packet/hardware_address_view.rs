use crate::errors::InvalidLength;
use crate::{dhcp_packet::HardwareAddress, errors::OutOfRange};
use core::borrow::{Borrow, BorrowMut};

#[derive(Debug, PartialEq, Eq)]
pub enum AddressError {
    InvalidLength(InvalidLength),
    OutOfRange(OutOfRange),
}

#[derive(Debug, PartialEq, Eq)]
pub struct HardwareAddressView<T> {
    data: T,
}

impl<T> HardwareAddressView<T>
where
    T: Borrow<[u8]>,
{
    #[inline(always)]
    pub(crate) fn from(data: T) -> Self {
        Self { data }
    }

    #[inline(always)]
    fn get_ethernet(&self) -> Result<HardwareAddress, AddressError> {
        const LENGTH_OFFSET: usize = 1;
        const MAC_LENGTH: u8 = 6;
        let length = self.data.borrow()[LENGTH_OFFSET];
        if length != MAC_LENGTH {
            return Err(AddressError::InvalidLength(InvalidLength::new(
                MAC_LENGTH as usize,
                length as usize,
            )));
        }

        const DATA_OFFSET: usize = 27;
        let data = &self.data.borrow()[DATA_OFFSET..];
        Ok(HardwareAddress::Ethernet(
            data[..MAC_LENGTH as usize].try_into().unwrap(),
        ))
    }

    #[inline]
    pub fn try_get(&self) -> Result<HardwareAddress, AddressError> {
        const TYPE_OFFSET: usize = 0;
        let data = self.data.borrow();
        match data[TYPE_OFFSET] {
            1 => self.get_ethernet(),
            _ => Err(AddressError::OutOfRange(OutOfRange)),
        }
    }
}

impl<T> HardwareAddressView<T>
where
    T: BorrowMut<[u8]>,
{
    #[inline(always)]
    fn set_ethernet(&mut self, value: &[u8; 6]) {
        const TYPE_OFFSET: usize = 0;
        const LENGTH_OFFSET: usize = 1;
        const DATA_OFFSET: usize = 27;
        let data = self.data.borrow_mut();
        data[TYPE_OFFSET] = 1;
        data[LENGTH_OFFSET] = value.len() as u8;
        data[DATA_OFFSET..].copy_from_slice(value);
    }

    #[inline]
    pub fn set(&mut self, value: HardwareAddress) {
        match value {
            HardwareAddress::Ethernet(x) => self.set_ethernet(&x),
        }
    }
}
