#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubnetAllocation;

impl SubnetAllocation {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
