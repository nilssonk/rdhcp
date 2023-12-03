#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataSource;

impl DataSource {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
