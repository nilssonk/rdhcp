#[derive(Debug, Clone, PartialEq)]
pub struct PolicyFilter;
        
impl PolicyFilter {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
