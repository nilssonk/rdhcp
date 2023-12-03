#[derive(Debug, Clone, PartialEq)]
pub struct RdnssSelection;
        
impl RdnssSelection {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
