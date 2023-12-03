#[derive(Debug, Clone, PartialEq)]
pub struct TCode;
        
impl TCode {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
