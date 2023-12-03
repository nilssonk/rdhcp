#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientLastTransactionTime;

impl ClientLastTransactionTime {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
