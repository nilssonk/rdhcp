#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UuidGuid;

impl UuidGuid {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
