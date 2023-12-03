#[derive(Debug, Clone, PartialEq)]
pub struct DhcpErrorMessage;
        
impl DhcpErrorMessage {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
