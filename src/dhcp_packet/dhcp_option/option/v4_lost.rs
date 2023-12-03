#[derive(Debug, Clone, PartialEq, Eq)]
pub struct V4Lost;

impl V4Lost {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
