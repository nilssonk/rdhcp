#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaskDiscovery;

impl MaskDiscovery {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
