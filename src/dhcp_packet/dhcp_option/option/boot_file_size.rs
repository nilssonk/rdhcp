#[derive(Debug, Clone, PartialEq)]
pub struct BootFileSize;
        
impl BootFileSize {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
