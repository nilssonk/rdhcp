#[derive(Debug, Clone, PartialEq)]
pub struct BaseTime;
        
impl BaseTime {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
