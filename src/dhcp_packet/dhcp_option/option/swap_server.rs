#[derive(Debug, Clone, PartialEq)]
pub struct SwapServer;
        
impl SwapServer {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
