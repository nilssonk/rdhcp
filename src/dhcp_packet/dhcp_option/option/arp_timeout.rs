#[derive(Debug, Clone, PartialEq)]
pub struct ArpTimeout;
        
impl ArpTimeout {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
