#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameServer;

impl NameServer {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
