#[derive(Debug, Clone, PartialEq)]
pub struct NisDomainName;
        
impl NisDomainName {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
