#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotesServer;

impl QuotesServer {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
