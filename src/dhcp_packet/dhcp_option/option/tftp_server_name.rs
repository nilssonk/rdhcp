#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TftpServerName;

impl TftpServerName {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
