#[derive(Debug, Clone, PartialEq)]
pub struct ForcerenewNonceCapable;
        
impl ForcerenewNonceCapable {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
