#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryStartTime;

impl QueryStartTime {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
