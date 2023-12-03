#[derive(Debug, Clone, PartialEq)]
pub struct V4Lost;

impl V4Lost {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
