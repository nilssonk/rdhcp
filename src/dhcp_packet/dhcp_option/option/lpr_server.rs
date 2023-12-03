#[derive(Debug, Clone, PartialEq)]
pub struct LprServer;
        
impl LprServer {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
