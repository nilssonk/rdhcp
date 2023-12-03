#[derive(Debug, Clone, PartialEq)]
pub struct IpAddressLeaseTime;
        
impl IpAddressLeaseTime {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
