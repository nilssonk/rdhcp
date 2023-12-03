#[derive(Debug, Clone, PartialEq)]
pub struct SubnetSelection;
        
impl SubnetSelection {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
