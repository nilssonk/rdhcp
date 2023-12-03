#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootPath;

impl RootPath {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
