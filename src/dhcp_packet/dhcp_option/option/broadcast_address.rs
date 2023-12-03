#[derive(Debug, Clone, PartialEq)]
pub struct BroadcastAddress;
        
impl BroadcastAddress {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
