#[derive(Debug, Clone, PartialEq)]
pub struct PcpServer;
        
impl PcpServer {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
