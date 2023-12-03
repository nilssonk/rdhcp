#[derive(Debug, Clone, PartialEq)]
pub struct NtpServers;
        
impl NtpServers {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
