#[derive(Debug, Clone, PartialEq)]
pub struct Unknown;
        
impl Unknown {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
