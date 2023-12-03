#[derive(Debug, Clone, PartialEq)]
pub struct MtuTimeout;
        
impl MtuTimeout {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
