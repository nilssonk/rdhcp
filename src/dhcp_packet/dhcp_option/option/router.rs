#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Router;

impl Router {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
