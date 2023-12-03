#[derive(Debug, Clone, PartialEq)]
pub struct Isns;
        
impl Isns {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
