#[derive(Debug, Clone, PartialEq)]
pub struct Removed;
        
impl Removed {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
