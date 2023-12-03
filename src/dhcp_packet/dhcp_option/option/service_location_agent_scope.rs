#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceLocationAgentScope;

impl ServiceLocationAgentScope {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
