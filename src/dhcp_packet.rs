use crate::errors::*;
use crate::DhcpOption;
use core::borrow::Borrow;
use std::net::IpAddr;

pub enum DhcpOperation {
    Request,
    Reply,
}

#[allow(dead_code)]
pub struct DhcpPacketFlags {
    data: u16,
}

impl DhcpPacketFlags {
    fn from(input: &[u8]) -> Self {
        Self {
            data: u16::from_be_bytes(input[..2].try_into().unwrap()),
        }
    }
}

#[derive(Debug)]
pub enum HardwareAddressType {
    Ethernet,
}

pub struct DhcpPacket<T> {
    data: T,
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

    pub fn get_client_ip() -> Option<IpAddr> {
        todo!()
    }

    pub fn get_offered_ip() -> Option<IpAddr> {
        todo!()
    }

    pub fn get_server_ip() -> Option<IpAddr> {
        todo!()
    }

    pub fn get_gateway_ip() -> Option<IpAddr> {
        todo!()
    }

    pub fn get_client_hardware_address() -> [u8; 16] {
        todo!()
    }

    pub fn get_server_name() -> [u8; 64] {
        todo!()
    }

    pub fn get_boot_file() -> [u8; 128] {
        todo!()
    }

    pub fn get_options() -> Vec<DhcpOption> {
        todo!()
    }
}
