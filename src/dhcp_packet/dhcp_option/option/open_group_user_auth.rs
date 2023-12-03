#[derive(Debug, Clone, PartialEq)]
pub struct OpenGroupUserAuth;
        
impl OpenGroupUserAuth {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
