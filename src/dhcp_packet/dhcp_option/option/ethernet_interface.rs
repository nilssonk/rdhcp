#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EthernetInterface;

impl EthernetInterface {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
