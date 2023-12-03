#[derive(Debug, Clone, PartialEq)]
pub struct Ieee8021QVlanId;

impl Ieee8021QVlanId {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
