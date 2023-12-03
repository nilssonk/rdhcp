#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyFilter;

impl PolicyFilter {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
