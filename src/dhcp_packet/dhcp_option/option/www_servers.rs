#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WwwServers;

impl WwwServers {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
