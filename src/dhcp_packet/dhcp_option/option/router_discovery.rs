#[derive(Debug, Clone, PartialEq)]
pub struct RouterDiscovery;
        
impl RouterDiscovery {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
