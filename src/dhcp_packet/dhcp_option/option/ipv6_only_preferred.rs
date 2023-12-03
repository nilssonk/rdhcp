#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ipv6OnlyPreferred;

impl Ipv6OnlyPreferred {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
