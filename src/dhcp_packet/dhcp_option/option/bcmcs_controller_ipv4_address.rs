#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BcmcsControllerIpv4Address;

impl BcmcsControllerIpv4Address {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
