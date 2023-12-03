#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetbiosName;

impl NetbiosName {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
