mod dhcp_operation;
pub use dhcp_operation::DhcpOperation;

mod dhcp_operation_view;
pub use dhcp_operation_view::DhcpOperationView;

pub mod dhcp_option;
pub use dhcp_option::DhcpOption;

mod dhcp_option_iterator;
pub use dhcp_option_iterator::DhcpOptionIterator;

mod dhcp_packet_flags_view;
pub use dhcp_packet_flags_view::DhcpPacketFlagsView;

mod hardware_address;
pub use hardware_address::HardwareAddress;

mod hardware_address_view;
pub use hardware_address_view::HardwareAddressView;

use crate::common::*;
use crate::errors::*;

pub use core::borrow::{Borrow, BorrowMut};
pub use core::iter::Iterator;

#[derive(Debug, PartialEq, Eq)]
pub struct DhcpPacket<T> {
    data: T,
}

impl<T> DhcpPacket<T>
where
    T: Borrow<[u8]>,
{
    #[inline]
    pub fn try_from(data: T) -> Result<Self, InvalidLength> {
        const MIN_SIZE: usize = 236;
        let slice = data.borrow();
        if slice.len() >= MIN_SIZE {
            Ok(Self { data })
        } else {
            Err(InvalidLength::new(MIN_SIZE, slice.len()))
        }
    }

    #[inline]
    pub fn operation(&self) -> DhcpOperationView<impl Borrow<[u8]> + '_> {
        const OFFSET: usize = 0;
        let data = self.data.borrow();
        DhcpOperationView::from(&data[OFFSET..])
    }

    #[inline]
    pub fn client_hardware_address(&self) -> HardwareAddressView<impl Borrow<[u8]> + '_> {
        const OFFSET: usize = 1;
        let data = self.data.borrow();
        HardwareAddressView::from(&data[OFFSET..])
    }

    #[inline]
    pub fn hops(&self) -> integer_view! {u8} {
        const OFFSET: usize = 3;
        let data = &self.data.borrow()[OFFSET..];
        IntegerView::from(&data[..1])
    }

    #[inline]
    pub fn xid(&self) -> integer_view! {u32} {
        const OFFSET: usize = 4;
        let data = &self.data.borrow()[OFFSET..];
        IntegerView::from(&data[..4])
    }

    #[inline]
    pub fn secs(&self) -> integer_view! {u16} {
        const OFFSET: usize = 8;
        let data = &self.data.borrow()[OFFSET..];
        IntegerView::from(&data[..2])
    }

    #[inline]
    pub fn flags(&self) -> DhcpPacketFlagsView<impl Borrow<[u8]> + '_> {
        const OFFSET: usize = 10;
        let data = &self.data.borrow()[OFFSET..];
        DhcpPacketFlagsView::from(&data[..2])
    }

    #[inline(always)]
    fn ip<const OFFSET: usize>(&self) -> ipv4_addr_view! {} {
        let data = &self.data.borrow()[OFFSET..];
        Ipv4AddrView::from(&data[..4])
    }

    #[inline]
    pub fn client_ip(&self) -> ipv4_addr_view! {} {
        const OFFSET: usize = 12;
        self.ip::<OFFSET>()
    }

    #[inline]
    pub fn offered_ip(&self) -> ipv4_addr_view! {} {
        const OFFSET: usize = 16;
        self.ip::<OFFSET>()
    }

    #[inline]
    pub fn server_ip(&self) -> ipv4_addr_view! {} {
        const OFFSET: usize = 20;
        self.ip::<OFFSET>()
    }

    #[inline]
    pub fn gateway_ip(&self) -> ipv4_addr_view! {} {
        const OFFSET: usize = 24;
        self.ip::<OFFSET>()
    }

    #[inline]
    pub fn server_name(&self) -> [u8; 64] {
        const OFFSET: usize = 44;
        const LENGTH: usize = 64;
        let data = &self.data.borrow()[OFFSET..];
        data[..LENGTH].try_into().unwrap()
    }

    #[inline]
    pub fn boot_file(&self) -> [u8; 128] {
        const OFFSET: usize = 108;
        const LENGTH: usize = 128;
        let data = &self.data.borrow()[OFFSET..];
        data[..LENGTH].try_into().unwrap()
    }

    #[inline]
    pub fn options(&self) -> impl Iterator<Item = DhcpOption> + '_ {
        const OFFSET: usize = 236;
        DhcpOptionIterator::from(&self.data.borrow()[OFFSET..])
    }
}

impl<T> DhcpPacket<T>
where
    T: BorrowMut<[u8]>,
{
    #[inline]
    pub fn operation_mut(&mut self) -> DhcpOperationView<impl BorrowMut<[u8]> + '_> {
        const OFFSET: usize = 0;
        let data = self.data.borrow_mut();
        DhcpOperationView::from(&mut data[OFFSET..])
    }

    #[inline]
    pub fn client_hardware_address_mut(
        &mut self,
    ) -> HardwareAddressView<impl BorrowMut<[u8]> + '_> {
        const OFFSET: usize = 1;
        let data = self.data.borrow_mut();
        HardwareAddressView::from(&mut data[OFFSET..])
    }

    #[inline(always)]
    fn integer_mut<const OFFSET: usize, U>(&mut self) -> integer_view_mut! {U}
    where
        U: BeInteger + SizedInteger,
    {
        let data = &mut self.data.borrow_mut()[OFFSET..];
        IntegerView::from_mut(&mut data[..U::SIZE])
    }

    #[inline]
    pub fn hops_mut(&mut self) -> integer_view_mut! {u8} {
        const OFFSET: usize = 3;
        self.integer_mut::<OFFSET, u8>()
    }

    #[inline]
    pub fn xid_mut(&mut self) -> integer_view_mut! {u32} {
        const OFFSET: usize = 4;
        self.integer_mut::<OFFSET, u32>()
    }

    #[inline]
    pub fn secs_mut(&mut self) -> integer_view_mut! {u16} {
        const OFFSET: usize = 8;
        self.integer_mut::<OFFSET, u16>()
    }

    #[inline]
    pub fn flags_mut(&mut self) -> DhcpPacketFlagsView<impl BorrowMut<[u8]> + '_> {
        const OFFSET: usize = 10;
        let data = &mut self.data.borrow_mut()[OFFSET..];
        DhcpPacketFlagsView::from_mut(&mut data[..2])
    }

    #[inline(always)]
    fn ip_mut<const OFFSET: usize>(&mut self) -> ipv4_addr_view_mut! {} {
        let data = &mut self.data.borrow_mut()[OFFSET..];
        Ipv4AddrView::from_mut(&mut data[..4])
    }

    #[inline]
    pub fn client_ip_mut(&mut self) -> ipv4_addr_view_mut! {} {
        const OFFSET: usize = 12;
        self.ip_mut::<OFFSET>()
    }

    #[inline]
    pub fn offered_ip_mut(&mut self) -> ipv4_addr_view_mut! {} {
        const OFFSET: usize = 16;
        self.ip_mut::<OFFSET>()
    }

    #[inline]
    pub fn server_ip_mut(&mut self) -> ipv4_addr_view_mut! {} {
        const OFFSET: usize = 20;
        self.ip_mut::<OFFSET>()
    }

    #[inline]
    pub fn gateway_ip_mut(&mut self) -> ipv4_addr_view_mut! {} {
        const OFFSET: usize = 24;
        self.ip_mut::<OFFSET>()
    }

    #[inline]
    pub fn server_name_mut(&self) -> [u8; 64] {
        const OFFSET: usize = 44;
        const LENGTH: usize = 64;
        let data = &self.data.borrow()[OFFSET..];
        data[..LENGTH].try_into().unwrap()
    }

    #[inline]
    pub fn boot_file_mut(&self) -> [u8; 128] {
        const OFFSET: usize = 108;
        const LENGTH: usize = 128;
        let data = &self.data.borrow()[OFFSET..];
        data[..LENGTH].try_into().unwrap()
    }

    #[inline]
    pub fn options_mut(&self) -> impl Iterator<Item = DhcpOption> + '_ {
        const OFFSET: usize = 236;
        DhcpOptionIterator::from(&self.data.borrow()[OFFSET..])
    }
}
