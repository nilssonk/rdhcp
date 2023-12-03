#[derive(Debug, Clone, PartialEq)]
pub struct NntpServers;
        
impl NntpServers {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
