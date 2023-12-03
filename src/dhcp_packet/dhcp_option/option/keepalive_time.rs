#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeepaliveTime;

impl KeepaliveTime {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
