#[derive(Debug, Clone, PartialEq)]
pub struct RebootTime;
        
impl RebootTime {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
