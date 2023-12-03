#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationFile;

impl ConfigurationFile {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
