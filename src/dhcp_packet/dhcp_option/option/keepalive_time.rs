#[derive(Debug, Clone, PartialEq)]
pub struct KeepaliveTime;
        
impl KeepaliveTime {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
