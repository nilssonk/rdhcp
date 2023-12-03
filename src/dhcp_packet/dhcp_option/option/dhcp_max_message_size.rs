#[derive(Debug, Clone, PartialEq)]
pub struct DhcpMaxMessageSize;
        
impl DhcpMaxMessageSize {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
