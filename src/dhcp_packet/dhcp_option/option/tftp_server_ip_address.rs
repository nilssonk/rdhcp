#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TftpServerIpAddress;

impl TftpServerIpAddress {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
