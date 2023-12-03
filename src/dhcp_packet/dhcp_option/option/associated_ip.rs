#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssociatedIp;

impl AssociatedIp {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
