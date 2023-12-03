#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogServer;

impl LogServer {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
