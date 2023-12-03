#[derive(Debug, Clone, PartialEq)]
pub struct StaticRoute;
        
impl StaticRoute {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
