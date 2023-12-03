#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImpressServer;

impl ImpressServer {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
