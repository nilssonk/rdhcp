#[derive(Debug, Clone, PartialEq)]
pub struct DhcpV4OverDhcpV6;
        
impl DhcpV4OverDhcpV6 {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
