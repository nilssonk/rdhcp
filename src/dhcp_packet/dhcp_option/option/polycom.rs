#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polycom;

impl Polycom {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
