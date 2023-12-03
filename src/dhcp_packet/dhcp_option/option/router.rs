#[derive(Debug, Clone, PartialEq)]
pub struct Router;
        
impl Router {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
