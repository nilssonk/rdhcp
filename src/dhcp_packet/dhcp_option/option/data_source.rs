#[derive(Debug, Clone, PartialEq)]
pub struct DataSource;
        
impl DataSource {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
