#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusCode;

impl StatusCode {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
