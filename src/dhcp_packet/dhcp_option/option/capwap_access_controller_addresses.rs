#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapwapAccessControllerAddresses;

impl CapwapAccessControllerAddresses {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
