#[derive(Debug, Clone, PartialEq)]
pub struct Lost;
        
impl Lost {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
