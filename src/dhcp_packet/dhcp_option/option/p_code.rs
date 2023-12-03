#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PCode;

impl PCode {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
