#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryEndTime;

impl QueryEndTime {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
