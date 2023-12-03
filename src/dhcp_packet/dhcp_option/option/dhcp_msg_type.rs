#[derive(Debug, Clone, PartialEq)]
pub struct DhcpMsgType;
        
impl DhcpMsgType {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
