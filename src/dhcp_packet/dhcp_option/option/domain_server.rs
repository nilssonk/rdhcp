#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainServer;

impl DomainServer {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
