#[derive(Debug, Clone, PartialEq)]
pub struct WwwServers;
        
impl WwwServers {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
