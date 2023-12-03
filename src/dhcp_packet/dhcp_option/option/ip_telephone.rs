#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpTelephone;

impl IpTelephone {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
