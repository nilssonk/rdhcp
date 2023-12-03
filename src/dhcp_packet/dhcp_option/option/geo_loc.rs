#[derive(Debug, Clone, PartialEq)]
pub struct GeoLoc;
        
impl GeoLoc {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
