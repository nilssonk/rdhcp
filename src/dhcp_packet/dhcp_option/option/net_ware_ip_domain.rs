#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetWareIpDomain;

impl NetWareIpDomain {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
