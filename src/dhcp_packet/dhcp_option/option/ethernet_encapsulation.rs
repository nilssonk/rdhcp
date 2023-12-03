#[derive(Debug, Clone, PartialEq)]
pub struct EthernetEncapsulation;
        
impl EthernetEncapsulation {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
