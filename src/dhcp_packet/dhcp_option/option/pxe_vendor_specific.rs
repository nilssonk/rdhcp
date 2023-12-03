#[derive(Debug, Clone, PartialEq)]
pub struct PxeVendorSpecific;
        
impl PxeVendorSpecific {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
