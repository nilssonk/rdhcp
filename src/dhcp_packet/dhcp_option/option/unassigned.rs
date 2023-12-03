#[derive(Debug, Clone, PartialEq)]
pub struct Unassigned;
        
impl Unassigned {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
