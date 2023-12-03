#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address;

impl Address {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
