#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryAgentInformation;

impl DirectoryAgentInformation {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
