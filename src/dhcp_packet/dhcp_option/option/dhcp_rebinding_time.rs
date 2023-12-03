#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DhcpRebindingTime;

impl DhcpRebindingTime {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
