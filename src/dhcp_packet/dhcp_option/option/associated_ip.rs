#[derive(Debug, Clone, PartialEq)]
pub struct AssociatedIp;
        
impl AssociatedIp {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
