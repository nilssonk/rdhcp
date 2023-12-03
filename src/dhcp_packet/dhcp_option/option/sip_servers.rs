#[derive(Debug, Clone, PartialEq)]
pub struct SipServers;
        
impl SipServers {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
