#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRoutingOnOff;

impl SourceRoutingOnOff {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
