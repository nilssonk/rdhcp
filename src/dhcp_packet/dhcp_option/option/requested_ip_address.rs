#[derive(Debug, Clone, PartialEq)]
pub struct RequestedIpAddress;
        
impl RequestedIpAddress {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
