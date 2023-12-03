#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PxeVendorSpecific;

impl PxeVendorSpecific {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
