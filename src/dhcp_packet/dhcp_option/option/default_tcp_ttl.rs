#[derive(Debug, Clone, PartialEq)]
pub struct DefaultTcpTtl;
        
impl DefaultTcpTtl {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
