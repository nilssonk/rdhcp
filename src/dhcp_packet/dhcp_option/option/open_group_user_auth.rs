#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenGroupUserAuth;

impl OpenGroupUserAuth {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
