#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ieee8021QVlanId;

impl Ieee8021QVlanId {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
