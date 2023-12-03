#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubnetSelection;

impl SubnetSelection {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
