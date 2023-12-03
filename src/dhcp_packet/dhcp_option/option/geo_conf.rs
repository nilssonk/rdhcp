#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeoConf;

impl GeoConf {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
