#[derive(Debug, Clone, PartialEq)]
pub struct NisDomain;
        
impl NisDomain {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
