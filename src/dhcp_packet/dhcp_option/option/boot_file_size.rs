#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootFileSize;

impl BootFileSize {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
