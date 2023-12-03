#[derive(Debug, Clone, PartialEq)]
pub struct DirectoryAgentInformation;
        
impl DirectoryAgentInformation {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
