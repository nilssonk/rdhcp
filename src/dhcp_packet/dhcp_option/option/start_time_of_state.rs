#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartTimeOfState;

impl StartTimeOfState {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
