#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TftpServerAddress;

impl TftpServerAddress {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
