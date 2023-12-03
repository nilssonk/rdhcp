#[derive(Debug, Clone, PartialEq)]
pub struct NisServerAddress;
        
impl NisServerAddress {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
