#[derive(Debug, Clone, PartialEq)]
pub struct DefaultIpTtl;
        
impl DefaultIpTtl {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
