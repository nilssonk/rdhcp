#[derive(Debug, Clone, PartialEq)]
pub struct ClientId;
        
impl ClientId {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
