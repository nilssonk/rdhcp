#[derive(Debug, Clone, PartialEq)]
pub struct EthernetInterface;
        
impl EthernetInterface {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
