#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DhcpErrorMessage;

impl DhcpErrorMessage {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
