#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DhcpState;

impl DhcpState {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
