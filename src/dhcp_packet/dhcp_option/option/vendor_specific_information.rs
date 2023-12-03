#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VendorSpecificInformation;

impl VendorSpecificInformation {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
