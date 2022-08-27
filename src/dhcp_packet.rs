use crate::DhcpOption;
use std::net::IpAddr;

pub enum DhcpPacketType {
    Request,
    Reply,
}

pub enum DhcpPacketFlags {
    // @TODO
}

#[allow(dead_code)] // Remove upon first use
pub struct DhcpPacket<T> {
    data: T,
}

impl<T> DhcpPacket<T> {
    pub fn get_type() -> DhcpPacketType {
        todo!()
    }

    pub fn get_length() -> u8 {
        todo!()
    }

    pub fn get_hops() -> u8 {
        todo!()
    }

    pub fn get_xid() -> u32 {
        todo!()
    }

    pub fn get_secs() -> u16 {
        todo!()
    }

    pub fn get_flags() -> DhcpPacketFlags {
        todo!()
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
