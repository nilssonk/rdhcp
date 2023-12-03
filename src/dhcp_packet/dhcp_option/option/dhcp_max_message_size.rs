#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DhcpMaxMessageSize;

impl DhcpMaxMessageSize {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
