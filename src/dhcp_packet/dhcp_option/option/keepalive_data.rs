#[derive(Debug, Clone, PartialEq)]
pub struct KeepaliveData;
        
impl KeepaliveData {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
