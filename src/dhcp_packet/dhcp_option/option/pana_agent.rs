#[derive(Debug, Clone, PartialEq)]
pub struct PanaAgent;
        
impl PanaAgent {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
