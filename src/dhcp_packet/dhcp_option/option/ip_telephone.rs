#[derive(Debug, Clone, PartialEq)]
pub struct IpTelephone;
        
impl IpTelephone {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
