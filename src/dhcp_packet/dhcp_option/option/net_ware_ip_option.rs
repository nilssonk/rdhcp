#[derive(Debug, Clone, PartialEq)]
pub struct NetWareIpOption;
        
impl NetWareIpOption {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
