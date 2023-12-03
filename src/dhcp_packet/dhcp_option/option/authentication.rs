#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Authentication;

impl Authentication {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
