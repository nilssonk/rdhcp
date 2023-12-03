#[derive(Debug, Clone, PartialEq)]
pub struct Hostname;
        
impl Hostname {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
