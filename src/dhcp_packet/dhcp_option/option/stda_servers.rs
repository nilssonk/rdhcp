#[derive(Debug, Clone, PartialEq)]
pub struct StdaServers;
        
impl StdaServers {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
