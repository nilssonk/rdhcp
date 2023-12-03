#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unassigned;

impl Unassigned {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
