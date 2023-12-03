#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HomeAgentAddress;

impl HomeAgentAddress {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
