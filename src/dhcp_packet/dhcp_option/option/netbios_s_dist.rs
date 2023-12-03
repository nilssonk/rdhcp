#[derive(Debug, Clone, PartialEq)]
pub struct NetbiosSDist;
        
impl NetbiosSDist {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
