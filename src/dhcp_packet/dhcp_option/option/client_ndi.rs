#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientNdi;

impl ClientNdi {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
