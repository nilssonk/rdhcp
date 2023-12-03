#[derive(Debug, Clone, PartialEq)]
pub struct NisServers;
        
impl NisServers {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
