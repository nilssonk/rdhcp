#[derive(Debug, Clone, PartialEq)]
pub struct RapidCommit;
        
impl RapidCommit {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
