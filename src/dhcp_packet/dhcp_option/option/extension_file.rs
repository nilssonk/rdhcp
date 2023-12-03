#[derive(Debug, Clone, PartialEq)]
pub struct ExtensionFile;
        
impl ExtensionFile {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
