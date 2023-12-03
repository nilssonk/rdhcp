#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmtpServers;

impl SmtpServers {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
