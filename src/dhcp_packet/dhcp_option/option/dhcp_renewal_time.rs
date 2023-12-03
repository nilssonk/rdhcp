#[derive(Debug, Clone, PartialEq)]
pub struct DhcpRenewalTime;
        
impl DhcpRenewalTime {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
