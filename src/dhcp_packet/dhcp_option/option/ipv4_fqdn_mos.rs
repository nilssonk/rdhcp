#[derive(Debug, Clone, PartialEq)]
pub struct Ipv4FqdnMos;

impl Ipv4FqdnMos {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
