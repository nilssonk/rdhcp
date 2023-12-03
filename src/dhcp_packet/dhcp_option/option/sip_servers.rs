#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SipServers;

impl SipServers {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
