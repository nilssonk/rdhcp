#[derive(Debug, Clone, PartialEq)]
pub struct DiffServCodePoint;
        
impl DiffServCodePoint {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
