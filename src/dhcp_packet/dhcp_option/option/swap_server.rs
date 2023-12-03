#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwapServer;

impl SwapServer {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
