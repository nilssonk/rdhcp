#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubnetMask;

impl SubnetMask {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
