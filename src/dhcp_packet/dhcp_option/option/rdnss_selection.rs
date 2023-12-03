#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RdnssSelection;

impl RdnssSelection {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
