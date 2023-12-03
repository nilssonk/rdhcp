#[derive(Debug, Clone, PartialEq)]
pub struct ClasslessStaticRoute;
        
impl ClasslessStaticRoute {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
