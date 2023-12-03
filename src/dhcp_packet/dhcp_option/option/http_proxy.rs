#[derive(Debug, Clone, PartialEq)]
pub struct HttpProxy;
        
impl HttpProxy {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
