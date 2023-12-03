#[derive(Debug, Clone, PartialEq)]
pub struct RelayAgentInformation;
        
impl RelayAgentInformation {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
