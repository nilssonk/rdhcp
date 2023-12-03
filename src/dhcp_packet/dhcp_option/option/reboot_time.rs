#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RebootTime;

impl RebootTime {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
