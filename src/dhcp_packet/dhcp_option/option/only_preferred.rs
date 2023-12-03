#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnlyPreferred;

impl OnlyPreferred {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
