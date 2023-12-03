#[derive(Debug, Clone, PartialEq)]
pub struct CableLabsClientConfiguration;
        
impl CableLabsClientConfiguration {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
