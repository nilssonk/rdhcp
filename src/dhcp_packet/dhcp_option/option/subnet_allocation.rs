#[derive(Debug, Clone, PartialEq)]
pub struct SubnetAllocation;
        
impl SubnetAllocation {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
