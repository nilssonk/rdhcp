#[derive(Debug, Clone, PartialEq)]
pub struct GeoConf;
        
impl GeoConf {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
