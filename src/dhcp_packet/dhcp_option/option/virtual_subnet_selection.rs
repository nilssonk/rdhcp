#[derive(Debug, Clone, PartialEq)]
pub struct VirtualSubnetSelection;
        
impl VirtualSubnetSelection {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
