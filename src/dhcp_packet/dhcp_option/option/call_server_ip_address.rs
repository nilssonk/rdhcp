#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallServerIpAddress;

impl CallServerIpAddress {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
