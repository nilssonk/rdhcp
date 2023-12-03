#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DhcpRenewalTime;

impl DhcpRenewalTime {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
