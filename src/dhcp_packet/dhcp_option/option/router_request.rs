#[derive(Debug, Clone, PartialEq)]
pub struct RouterRequest;
        
impl RouterRequest {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
