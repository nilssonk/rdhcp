#[derive(Debug, Clone, PartialEq)]
pub struct StartTimeOfState;
        
impl StartTimeOfState {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}