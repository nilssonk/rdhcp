#[derive(Debug, Clone, PartialEq)]
pub struct LogServer;
        
impl LogServer {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
