#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessDomain;

impl AccessDomain {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
