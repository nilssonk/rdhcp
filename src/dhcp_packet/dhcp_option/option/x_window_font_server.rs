#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XWindowFontServer;

impl XWindowFontServer {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
