#[derive(Debug, Clone, PartialEq)]
pub struct QueryStartTime;
        
impl QueryStartTime {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
