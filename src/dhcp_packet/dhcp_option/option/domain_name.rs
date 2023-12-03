#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainName;

impl DomainName {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
