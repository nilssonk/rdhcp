#[derive(Debug, Clone, PartialEq)]
pub struct NetbiosScope;
        
impl NetbiosScope {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
