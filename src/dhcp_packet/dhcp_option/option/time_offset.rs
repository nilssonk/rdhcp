#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeOffset;

impl TimeOffset {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
