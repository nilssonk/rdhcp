#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ieee8021DLayer2Priority;

impl Ieee8021DLayer2Priority {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
