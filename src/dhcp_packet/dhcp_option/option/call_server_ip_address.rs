#[derive(Debug, Clone, PartialEq)]
pub struct CallServerIpAddress;
        
impl CallServerIpAddress {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
