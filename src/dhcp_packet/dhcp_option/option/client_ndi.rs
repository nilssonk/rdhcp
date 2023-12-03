#[derive(Debug, Clone, PartialEq)]
pub struct ClientNdi;
        
impl ClientNdi {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
