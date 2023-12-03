#[derive(Debug, Clone, PartialEq)]
pub struct MaskDiscovery;
        
impl MaskDiscovery {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
