#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeoLoc;

impl GeoLoc {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
