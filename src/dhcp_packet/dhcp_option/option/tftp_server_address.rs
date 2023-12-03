#[derive(Debug, Clone, PartialEq)]
pub struct TftpServerAddress;
        
impl TftpServerAddress {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
