pub mod dhcp_option;
pub use dhcp_option::DhcpOption;

pub mod dhcp_option_iterator;
pub use dhcp_option_iterator::DhcpOptionIterator;

use crate::errors::*;
pub use core::borrow::Borrow;
pub use core::iter::Iterator;
use std::net::Ipv4Addr;

#[derive(Debug, PartialEq, Eq)]
pub enum DhcpOperation {
    Request,
    Reply,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DhcpPacketFlags {
    data: u16,
}

impl DhcpPacketFlags {
    fn from(input: &[u8]) -> Self {
        Self {
            data: u16::from_be_bytes(input[..2].try_into().unwrap()),
        }
    }

    pub fn is_broadcast(&self) -> bool {
        self.data & 0x1 != 0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum HardwareAddressType {
    Ethernet,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DhcpPacket<T> {
    data: T,
}

impl<T> DhcpPacket<T> {
    pub fn from(data: T) -> Self {
        Self { data }
    }
}

impl<T> DhcpPacket<T>
where
    T: Borrow<[u8]>,
{
    pub fn get_operation(&self) -> Result<DhcpOperation, OutOfRange> {
        const OFFSET: usize = 0;

        let data = self.data.borrow();
        match data[OFFSET] {
            1 => Ok(DhcpOperation::Request),
            2 => Ok(DhcpOperation::Reply),
            _ => Err(OutOfRange),
        }
    }

    pub fn get_hardware_address_type(&self) -> Result<HardwareAddressType, OutOfRange> {
        const OFFSET: usize = 1;
        let data = self.data.borrow();
        match data[OFFSET] {
            1 => Ok(HardwareAddressType::Ethernet),
            _ => Err(OutOfRange),
        }
    }

    pub fn get_hardware_address_length(&self) -> u8 {
        const OFFSET: usize = 2;
        self.data.borrow()[OFFSET]
    }

    pub fn get_hops(&self) -> u8 {
        const OFFSET: usize = 3;
        self.data.borrow()[OFFSET]
    }

    pub fn get_xid(&self) -> u32 {
        const OFFSET: usize = 4;
        let data = &self.data.borrow()[OFFSET..];
        u32::from_be_bytes(data[..4].try_into().unwrap())
    }

    pub fn get_secs(&self) -> u16 {
        const OFFSET: usize = 8;
        let data = &self.data.borrow()[OFFSET..];
        u16::from_be_bytes(data[..2].try_into().unwrap())
    }

    pub fn get_flags(&self) -> DhcpPacketFlags {
        const OFFSET: usize = 10;
        let data = &self.data.borrow()[OFFSET..];
        DhcpPacketFlags::from(data)
    }

    #[inline(always)]
    fn get_ip<const OFFSET: usize>(&self) -> Ipv4Addr {
        let data = &self.data.borrow()[OFFSET..];
        let octets: [u8; 4] = data[..4].try_into().unwrap();
        octets.into()
    }

    pub fn get_client_ip(&self) -> Ipv4Addr {
        const OFFSET: usize = 12;
        self.get_ip::<OFFSET>()
    }

    pub fn get_offered_ip(&self) -> Ipv4Addr {
        const OFFSET: usize = 16;
        self.get_ip::<OFFSET>()
    }

    pub fn get_server_ip(&self) -> Ipv4Addr {
        const OFFSET: usize = 20;
        self.get_ip::<OFFSET>()
    }

    pub fn get_gateway_ip(&self) -> Ipv4Addr {
        const OFFSET: usize = 24;
        self.get_ip::<OFFSET>()
    }

    pub fn get_client_hardware_address(&self) -> Vec<u8> {
        const OFFSET: usize = 28;
        const MAX_LENGTH: u8 = 16;
        let length = std::cmp::min(MAX_LENGTH, self.get_hardware_address_length());
        let data = &self.data.borrow()[OFFSET..];
        data[..length as usize].into()
    }

    pub fn get_server_name(&self) -> [u8; 64] {
        const OFFSET: usize = 44;
        const LENGTH: usize = 64;
        let data = &self.data.borrow()[OFFSET..];
        data[..LENGTH].try_into().unwrap()
    }

    pub fn get_boot_file(&self) -> [u8; 128] {
        const OFFSET: usize = 108;
        const LENGTH: usize = 128;
        let data = &self.data.borrow()[OFFSET..];
        data[..LENGTH].try_into().unwrap()
    }

    pub fn get_options(&self) -> impl Iterator<Item = DhcpOption> + '_ {
        const OFFSET: usize = 236;
        DhcpOptionIterator::from(&self.data.borrow()[OFFSET..])
    }
}
