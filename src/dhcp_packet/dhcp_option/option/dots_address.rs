#[derive(Debug, Clone, PartialEq)]
pub struct DotsAddress;
        
impl DotsAddress {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
