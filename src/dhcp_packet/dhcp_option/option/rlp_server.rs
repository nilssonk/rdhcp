#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RlpServer;

impl RlpServer {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
