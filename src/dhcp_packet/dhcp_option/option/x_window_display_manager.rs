#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XWindowDisplayManager;

impl XWindowDisplayManager {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
