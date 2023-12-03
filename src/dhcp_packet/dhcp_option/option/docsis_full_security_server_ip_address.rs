#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocsisFullSecurityServerIpAddress;

impl DocsisFullSecurityServerIpAddress {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
