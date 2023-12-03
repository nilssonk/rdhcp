#[derive(Debug, Clone, PartialEq)]
pub struct XWindowFontServer;
        
impl XWindowFontServer {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
