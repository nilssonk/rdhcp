#[derive(Debug, Clone, PartialEq)]
pub struct NdsContext;
        
impl NdsContext {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
