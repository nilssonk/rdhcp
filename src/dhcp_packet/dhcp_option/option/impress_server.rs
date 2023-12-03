#[derive(Debug, Clone, PartialEq)]
pub struct ImpressServer;
        
impl ImpressServer {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
