pub mod dhcp_option;
pub use dhcp_option::DhcpOption;

mod dhcp_option_iterator;
pub use dhcp_option_iterator::DhcpOptionIterator;

mod dhcp_packet_flags_view;
pub use dhcp_packet_flags_view::DhcpPacketFlagsView;

use crate::common::*;
use crate::errors::*;

pub use core::borrow::{Borrow, BorrowMut};
pub use core::iter::Iterator;

#[derive(Debug, PartialEq, Eq)]
pub enum DhcpOperation {
    Request,
    Reply,
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

    pub fn get_hardware_address_length(&self) -> integer_view! {u8} {
        const OFFSET: usize = 2;
        let data = &self.data.borrow()[OFFSET..];
        IntegerView::from(&data[..1])
    }

    pub fn get_hops(&self) -> integer_view! {u8} {
        const OFFSET: usize = 3;
        let data = &self.data.borrow()[OFFSET..];
        IntegerView::from(&data[..1])
    }

    pub fn get_xid(&self) -> integer_view! {u32} {
        const OFFSET: usize = 4;
        let data = &self.data.borrow()[OFFSET..];
        IntegerView::from(&data[..4])
    }

    pub fn get_secs(&self) -> integer_view! {u16} {
        const OFFSET: usize = 8;
        let data = &self.data.borrow()[OFFSET..];
        IntegerView::from(&data[..2])
    }

    pub fn get_flags(&self) -> DhcpPacketFlagsView<impl Borrow<[u8]> + '_> {
        const OFFSET: usize = 10;
        let data = &self.data.borrow()[OFFSET..];
        DhcpPacketFlagsView::from(&data[..2])
    }

    #[inline(always)]
    fn get_ip<const OFFSET: usize>(&self) -> ipv4_addr_view! {} {
        let data = &self.data.borrow()[OFFSET..];
        Ipv4AddrView::from(&data[..4])
    }

    pub fn get_client_ip(&self) -> ipv4_addr_view! {} {
        const OFFSET: usize = 12;
        self.get_ip::<OFFSET>()
    }

    pub fn get_offered_ip(&self) -> ipv4_addr_view! {} {
        const OFFSET: usize = 16;
        self.get_ip::<OFFSET>()
    }

    pub fn get_server_ip(&self) -> ipv4_addr_view! {} {
        const OFFSET: usize = 20;
        self.get_ip::<OFFSET>()
    }

    pub fn get_gateway_ip(&self) -> ipv4_addr_view! {} {
        const OFFSET: usize = 24;
        self.get_ip::<OFFSET>()
    }

    pub fn get_client_hardware_address(&self) -> &[u8] {
        const OFFSET: usize = 28;
        const MAX_LENGTH: u8 = 16;

        let stated_address_length = self.get_hardware_address_length().get();
        let length = std::cmp::min(MAX_LENGTH, stated_address_length);

        let data = &self.data.borrow()[OFFSET..];
        &data[..length as usize]
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

impl<T> DhcpPacket<T>
where
    T: BorrowMut<[u8]>,
{
    pub fn get_operation_mut(&self) -> Result<DhcpOperation, OutOfRange> {
        const OFFSET: usize = 0;

        let data = self.data.borrow();
        match data[OFFSET] {
            1 => Ok(DhcpOperation::Request),
            2 => Ok(DhcpOperation::Reply),
            _ => Err(OutOfRange),
        }
    }

    pub fn get_hardware_address_type_mut(&self) -> Result<HardwareAddressType, OutOfRange> {
        const OFFSET: usize = 1;
        let data = self.data.borrow();
        match data[OFFSET] {
            1 => Ok(HardwareAddressType::Ethernet),
            _ => Err(OutOfRange),
        }
    }

    #[inline(always)]
    fn get_integer_mut<const OFFSET: usize, U>(&mut self) -> integer_view_mut! {U}
    where
        U: BeInteger + SizedInteger,
    {
        let data = &mut self.data.borrow_mut()[OFFSET..];
        IntegerView::from_mut(&mut data[..U::SIZE])
    }

    pub fn get_hardware_address_length_mut(&mut self) -> integer_view_mut! {u8} {
        const OFFSET: usize = 2;
        self.get_integer_mut::<OFFSET, u8>()
    }

    pub fn get_hops_mut(&mut self) -> integer_view_mut! {u8} {
        const OFFSET: usize = 3;
        self.get_integer_mut::<OFFSET, u8>()
    }

    pub fn get_xid_mut(&mut self) -> integer_view_mut! {u32} {
        const OFFSET: usize = 4;
        self.get_integer_mut::<OFFSET, u32>()
    }

    pub fn get_secs_mut(&mut self) -> integer_view_mut! {u16} {
        const OFFSET: usize = 8;
        self.get_integer_mut::<OFFSET, u16>()
    }

    pub fn get_flags_mut(&mut self) -> DhcpPacketFlagsView<impl BorrowMut<[u8]> + '_> {
        const OFFSET: usize = 10;
        let data = &mut self.data.borrow_mut()[OFFSET..];
        DhcpPacketFlagsView::from_mut(&mut data[..2])
    }

    #[inline(always)]
    fn get_ip_mut<const OFFSET: usize>(&mut self) -> ipv4_addr_view_mut! {} {
        let data = &mut self.data.borrow_mut()[OFFSET..];
        Ipv4AddrView::from_mut(&mut data[..4])
    }

    pub fn get_client_ip_mut(&mut self) -> ipv4_addr_view_mut! {} {
        const OFFSET: usize = 12;
        self.get_ip_mut::<OFFSET>()
    }

    pub fn get_offered_ip_mut(&mut self) -> ipv4_addr_view_mut! {} {
        const OFFSET: usize = 16;
        self.get_ip_mut::<OFFSET>()
    }

    pub fn get_server_ip_mut(&mut self) -> ipv4_addr_view_mut! {} {
        const OFFSET: usize = 20;
        self.get_ip_mut::<OFFSET>()
    }

    pub fn get_gateway_ip_mut(&mut self) -> ipv4_addr_view_mut! {} {
        const OFFSET: usize = 24;
        self.get_ip_mut::<OFFSET>()
    }

    pub fn get_client_hardware_address_mut(&mut self) -> &mut [u8] {
        const OFFSET: usize = 28;
        const MAX_LENGTH: u8 = 16;
        let stated_address_length = self.get_hardware_address_length().get();
        let length = std::cmp::min(MAX_LENGTH, stated_address_length);

        let data = &mut self.data.borrow_mut()[OFFSET..];
        &mut data[..length as usize]
    }

    pub fn get_server_name_mut(&self) -> [u8; 64] {
        const OFFSET: usize = 44;
        const LENGTH: usize = 64;
        let data = &self.data.borrow()[OFFSET..];
        data[..LENGTH].try_into().unwrap()
    }

    pub fn get_boot_file_mut(&self) -> [u8; 128] {
        const OFFSET: usize = 108;
        const LENGTH: usize = 128;
        let data = &self.data.borrow()[OFFSET..];
        data[..LENGTH].try_into().unwrap()
    }

    pub fn get_options_mut(&self) -> impl Iterator<Item = DhcpOption> + '_ {
        const OFFSET: usize = 236;
        DhcpOptionIterator::from(&self.data.borrow()[OFFSET..])
    }
}
