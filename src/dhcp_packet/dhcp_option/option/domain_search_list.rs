#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainSearchList;

impl DomainSearchList {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
