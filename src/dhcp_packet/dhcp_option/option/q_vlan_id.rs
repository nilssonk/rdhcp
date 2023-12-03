#[derive(Debug, Clone, PartialEq)]
pub struct QVlanId;
        
impl QVlanId {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
