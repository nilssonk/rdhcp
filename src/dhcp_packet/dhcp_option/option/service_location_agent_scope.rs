#[derive(Debug, Clone, PartialEq)]
pub struct ServiceLocationAgentScope;
        
impl ServiceLocationAgentScope {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
