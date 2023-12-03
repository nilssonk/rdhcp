#[derive(Debug, Clone, PartialEq)]
pub struct BootFileName;
        
impl BootFileName {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
