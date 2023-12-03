#[derive(Debug, Clone, PartialEq)]
pub struct DomainSearchList;
        
impl DomainSearchList {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
