#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PanaAgent;

impl PanaAgent {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
