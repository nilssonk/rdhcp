#[derive(Debug, Clone, PartialEq)]
pub struct UserClassInformation;
        
impl UserClassInformation {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
