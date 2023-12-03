#[derive(Debug, Clone, PartialEq)]
pub struct NetinfoTag;
        
impl NetinfoTag {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
