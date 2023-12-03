#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientFqdn;

impl ClientFqdn {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
