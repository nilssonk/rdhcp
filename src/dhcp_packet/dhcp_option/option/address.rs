#[derive(Debug, Clone, PartialEq)]
pub struct Address;
        
impl Address {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
