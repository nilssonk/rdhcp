#[derive(Debug, Clone, PartialEq)]
pub struct RootPath;
        
impl RootPath {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
