#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Overload;

impl Overload {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
