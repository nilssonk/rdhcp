#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelayAgentInformation;

impl RelayAgentInformation {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
