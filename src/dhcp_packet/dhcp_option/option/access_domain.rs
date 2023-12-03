#[derive(Debug, Clone, PartialEq)]
pub struct AccessDomain;
        
impl AccessDomain {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
