#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Isns;

impl Isns {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
