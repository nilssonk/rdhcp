#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ipv4AddressAndsf;

impl Ipv4AddressAndsf {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
