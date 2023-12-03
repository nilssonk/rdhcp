#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MtuTimeout;

impl MtuTimeout {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
