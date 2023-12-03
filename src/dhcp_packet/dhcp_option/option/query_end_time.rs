#[derive(Debug, Clone, PartialEq)]
pub struct QueryEndTime;
        
impl QueryEndTime {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
