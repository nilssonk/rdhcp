#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DhcpCaptivePortal;

impl DhcpCaptivePortal {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
