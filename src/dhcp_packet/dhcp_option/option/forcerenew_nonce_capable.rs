#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForcerenewNonceCapable;

impl ForcerenewNonceCapable {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
