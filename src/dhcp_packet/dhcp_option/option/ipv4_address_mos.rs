#[derive(Debug, Clone, PartialEq)]
pub struct Ipv4AddressMos;

impl Ipv4AddressMos {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
