#[derive(Debug, Clone, PartialEq)]
pub struct Padding;
        
impl Padding {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
