#[derive(Debug, Clone, PartialEq)]
pub struct StatusCode;
        
impl StatusCode {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
