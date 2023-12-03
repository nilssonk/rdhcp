#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClasslessStaticRoute;

impl ClasslessStaticRoute {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
