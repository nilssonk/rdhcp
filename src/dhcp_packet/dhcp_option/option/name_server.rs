#[derive(Debug, Clone, PartialEq)]
pub struct NameServer;
        
impl NameServer {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
