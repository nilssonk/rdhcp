#[derive(Debug, Clone, PartialEq)]
pub struct MtuInterface;
        
impl MtuInterface {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
