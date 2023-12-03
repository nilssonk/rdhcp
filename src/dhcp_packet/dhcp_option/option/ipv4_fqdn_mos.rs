#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ipv4FqdnMos;

impl Ipv4FqdnMos {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
