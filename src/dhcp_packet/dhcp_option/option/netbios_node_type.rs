#[derive(Debug, Clone, PartialEq)]
pub struct NetbiosNodeType;
        
impl NetbiosNodeType {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
