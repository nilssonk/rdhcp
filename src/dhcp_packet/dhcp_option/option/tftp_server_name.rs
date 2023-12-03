#[derive(Debug, Clone, PartialEq)]
pub struct TftpServerName;
        
impl TftpServerName {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
