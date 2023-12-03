#[derive(Debug, Clone, PartialEq)]
pub struct ClientSystem;
        
impl ClientSystem {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
