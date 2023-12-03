#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientSystem;

impl ClientSystem {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
