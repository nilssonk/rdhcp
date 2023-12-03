#[derive(Debug, Clone, PartialEq)]
pub struct ClientFqdn;
        
impl ClientFqdn {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
