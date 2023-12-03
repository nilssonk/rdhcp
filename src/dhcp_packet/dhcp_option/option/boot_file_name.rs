#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootFileName;

impl BootFileName {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
