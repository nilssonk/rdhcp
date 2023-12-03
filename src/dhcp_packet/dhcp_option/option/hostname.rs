#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hostname;

impl Hostname {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
