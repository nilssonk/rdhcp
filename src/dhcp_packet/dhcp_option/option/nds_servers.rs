#[derive(Debug, Clone, PartialEq)]
pub struct NdsServers;
        
impl NdsServers {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
