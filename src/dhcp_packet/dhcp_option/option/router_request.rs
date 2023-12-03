#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouterRequest;

impl RouterRequest {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
