#[derive(Debug, Clone, PartialEq)]
pub struct Authentication;
        
impl Authentication {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
