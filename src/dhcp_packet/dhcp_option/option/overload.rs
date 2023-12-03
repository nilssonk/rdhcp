#[derive(Debug, Clone, PartialEq)]
pub struct Overload;
        
impl Overload {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
