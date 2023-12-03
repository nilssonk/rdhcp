#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ipv4AddressMos;

impl Ipv4AddressMos {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
