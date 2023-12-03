#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestedIpAddress;

impl RequestedIpAddress {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
