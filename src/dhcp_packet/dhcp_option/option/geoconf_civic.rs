#[derive(Debug, Clone, PartialEq)]
pub struct GeoconfCivic;
        
impl GeoconfCivic {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
