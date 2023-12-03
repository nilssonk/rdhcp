#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lost;

impl Lost {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
