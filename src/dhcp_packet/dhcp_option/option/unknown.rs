#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unknown;

impl Unknown {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
