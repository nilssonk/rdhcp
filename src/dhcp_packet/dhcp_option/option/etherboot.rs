#[derive(Debug, Clone, PartialEq)]
pub struct Etherboot;
        
impl Etherboot {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
