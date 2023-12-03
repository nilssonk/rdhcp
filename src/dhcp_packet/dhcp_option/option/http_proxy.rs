#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpProxy;

impl HttpProxy {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
