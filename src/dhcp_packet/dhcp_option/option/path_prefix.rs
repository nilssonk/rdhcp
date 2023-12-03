#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathPrefix;

impl PathPrefix {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
