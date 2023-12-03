#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NntpServers;

impl NntpServers {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
