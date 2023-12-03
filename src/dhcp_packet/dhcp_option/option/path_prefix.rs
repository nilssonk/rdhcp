#[derive(Debug, Clone, PartialEq)]
pub struct PathPrefix;
        
impl PathPrefix {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
