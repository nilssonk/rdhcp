#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DhcpAutoConfiguration;

impl DhcpAutoConfiguration {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
