#[derive(Debug, Clone, PartialEq)]
pub struct BcmcsControllerIpv4Address;

impl BcmcsControllerIpv4Address {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
