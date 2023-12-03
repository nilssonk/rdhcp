#[derive(Debug, Clone, PartialEq)]
pub struct TimeServer;
        
impl TimeServer {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
