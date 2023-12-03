#[derive(Debug, Clone, PartialEq)]
pub struct ClassId;
        
impl ClassId {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
