#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwardingOnOff;

impl ForwardingOnOff {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
