#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TCode;

impl TCode {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
