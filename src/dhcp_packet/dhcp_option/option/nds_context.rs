#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NdsContext;

impl NdsContext {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
