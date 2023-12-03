#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualSubnetSelection;

impl VirtualSubnetSelection {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
