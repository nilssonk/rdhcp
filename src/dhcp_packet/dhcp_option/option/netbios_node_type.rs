#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetbiosNodeType;

impl NetbiosNodeType {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
