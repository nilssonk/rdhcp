#[derive(Debug, Clone, PartialEq)]
pub struct Ipv6OnlyPreferred;

impl Ipv6OnlyPreferred {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
