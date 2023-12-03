#[derive(Debug, Clone, PartialEq)]
pub struct ForwardingOnOff;
        
impl ForwardingOnOff {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
