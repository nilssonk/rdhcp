#[derive(Debug, Clone, PartialEq)]
pub struct RlpServer;
        
impl RlpServer {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
