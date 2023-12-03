#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameServiceSearch;

impl NameServiceSearch {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
