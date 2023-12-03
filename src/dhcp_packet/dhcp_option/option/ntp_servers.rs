#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NtpServers;

impl NtpServers {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
