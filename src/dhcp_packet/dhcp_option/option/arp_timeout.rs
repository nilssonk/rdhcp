#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArpTimeout;

impl ArpTimeout {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
