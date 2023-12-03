#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetbiosScope;

impl NetbiosScope {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
