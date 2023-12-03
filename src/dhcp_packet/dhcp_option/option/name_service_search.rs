#[derive(Debug, Clone, PartialEq)]
pub struct NameServiceSearch;
        
impl NameServiceSearch {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
