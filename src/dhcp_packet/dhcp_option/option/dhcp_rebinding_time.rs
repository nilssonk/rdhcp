#[derive(Debug, Clone, PartialEq)]
pub struct DhcpRebindingTime;
        
impl DhcpRebindingTime {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
