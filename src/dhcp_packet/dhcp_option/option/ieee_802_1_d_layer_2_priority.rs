#[derive(Debug, Clone, PartialEq)]
pub struct Ieee8021DLayer2Priority;

impl Ieee8021DLayer2Priority {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
