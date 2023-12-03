#[derive(Debug, Clone, PartialEq)]
pub struct PCode;
        
impl PCode {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
