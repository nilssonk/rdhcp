#[derive(Debug, Clone, PartialEq)]
pub struct MtuSubnet;
        
impl MtuSubnet {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
