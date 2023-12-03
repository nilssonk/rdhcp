#[derive(Debug, Clone, PartialEq)]
pub struct DhcpState;
        
impl DhcpState {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
