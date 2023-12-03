#[derive(Debug, Clone, PartialEq)]
pub struct TftpServerIpAddress;
        
impl TftpServerIpAddress {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
