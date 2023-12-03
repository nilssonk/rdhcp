#[derive(Debug, Clone, PartialEq)]
pub struct DomainServer;
        
impl DomainServer {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
