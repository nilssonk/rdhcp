#[derive(Debug, Clone, PartialEq)]
pub struct EtherbootSignature;
        
impl EtherbootSignature {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
