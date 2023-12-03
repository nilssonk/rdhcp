#[derive(Debug, Clone, PartialEq)]
pub struct SourceRoutingOnOff;
        
impl SourceRoutingOnOff {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
