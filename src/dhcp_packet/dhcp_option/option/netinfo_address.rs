#[derive(Debug, Clone, PartialEq)]
pub struct NetinfoAddress;
        
impl NetinfoAddress {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
