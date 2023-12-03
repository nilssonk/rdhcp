#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PcpServer;

impl PcpServer {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
