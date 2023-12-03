#[derive(Debug, Clone, PartialEq)]
pub struct Reserved;
        
impl Reserved {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
