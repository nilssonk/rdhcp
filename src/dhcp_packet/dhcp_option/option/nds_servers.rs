#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NdsServers;

impl NdsServers {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
