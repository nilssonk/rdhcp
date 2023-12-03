#[derive(Debug, Clone, PartialEq)]
pub struct SmtpServers;
        
impl SmtpServers {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
