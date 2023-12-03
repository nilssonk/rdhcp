#[derive(Debug, Clone, PartialEq)]
pub struct Priority;
        
impl Priority {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
