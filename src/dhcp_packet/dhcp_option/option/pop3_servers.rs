#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pop3Servers;

impl Pop3Servers {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
