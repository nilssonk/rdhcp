#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DhcpV4OverDhcpV6;

impl DhcpV4OverDhcpV6 {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
