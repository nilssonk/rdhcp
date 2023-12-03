#[derive(Debug, Clone, PartialEq)]
pub struct SubnetMask;
        
impl SubnetMask {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
