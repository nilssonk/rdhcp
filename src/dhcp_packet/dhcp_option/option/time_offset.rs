#[derive(Debug, Clone, PartialEq)]
pub struct TimeOffset;
        
impl TimeOffset {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
