#[derive(Debug, Clone, PartialEq)]
pub struct MtuPlateau;
        
impl MtuPlateau {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
