#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassId;

impl ClassId {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
