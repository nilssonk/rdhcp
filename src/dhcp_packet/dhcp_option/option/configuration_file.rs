#[derive(Debug, Clone, PartialEq)]
pub struct ConfigurationFile;
        
impl ConfigurationFile {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
