#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reserved;

impl Reserved {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
