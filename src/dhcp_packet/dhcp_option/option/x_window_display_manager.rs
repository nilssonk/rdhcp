#[derive(Debug, Clone, PartialEq)]
pub struct XWindowDisplayManager;
        
impl XWindowDisplayManager {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
