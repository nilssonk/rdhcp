#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrAddresses6rd;

impl BrAddresses6rd {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
