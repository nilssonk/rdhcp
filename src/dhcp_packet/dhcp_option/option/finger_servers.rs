#[derive(Debug, Clone, PartialEq)]
pub struct FingerServers;
        
impl FingerServers {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
