#[derive(Debug, Clone, PartialEq)]
pub struct QuotesServer;
        
impl QuotesServer {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
