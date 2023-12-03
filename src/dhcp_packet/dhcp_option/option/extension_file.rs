#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionFile;

impl ExtensionFile {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
