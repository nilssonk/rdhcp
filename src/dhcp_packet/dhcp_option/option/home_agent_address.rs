#[derive(Debug, Clone, PartialEq)]
pub struct HomeAgentAddress;
        
impl HomeAgentAddress {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
