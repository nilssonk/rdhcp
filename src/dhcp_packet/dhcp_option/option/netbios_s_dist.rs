#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetbiosSDist;

impl NetbiosSDist {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
