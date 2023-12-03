#[derive(Debug, Clone, PartialEq)]
pub struct UuidGuid;
        
impl UuidGuid {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
