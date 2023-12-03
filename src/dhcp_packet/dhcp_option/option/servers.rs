#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Servers;

impl Servers {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
