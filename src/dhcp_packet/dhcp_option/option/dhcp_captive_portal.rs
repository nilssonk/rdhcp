#[derive(Debug, Clone, PartialEq)]
pub struct DhcpCaptivePortal;
        
impl DhcpCaptivePortal {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
