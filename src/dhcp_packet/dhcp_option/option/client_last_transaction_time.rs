#[derive(Debug, Clone, PartialEq)]
pub struct ClientLastTransactionTime;
        
impl ClientLastTransactionTime {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
