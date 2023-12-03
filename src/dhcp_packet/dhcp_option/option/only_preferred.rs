#[derive(Debug, Clone, PartialEq)]
pub struct OnlyPreferred;
        
impl OnlyPreferred {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
