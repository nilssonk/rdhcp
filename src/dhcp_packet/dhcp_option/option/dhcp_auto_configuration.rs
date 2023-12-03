#[derive(Debug, Clone, PartialEq)]
pub struct DhcpAutoConfiguration;
        
impl DhcpAutoConfiguration {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
