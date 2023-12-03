#[derive(Debug, Clone, PartialEq)]
pub struct DomainName;
        
impl DomainName {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
