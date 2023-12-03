#[derive(Debug, Clone, PartialEq)]
pub struct NetbiosName;
        
impl NetbiosName {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
