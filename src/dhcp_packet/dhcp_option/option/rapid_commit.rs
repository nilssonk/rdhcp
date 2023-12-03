#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RapidCommit;

impl RapidCommit {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
