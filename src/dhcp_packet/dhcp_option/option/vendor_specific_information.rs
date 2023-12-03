#[derive(Debug, Clone, PartialEq)]
pub struct VendorSpecificInformation;
        
impl VendorSpecificInformation {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
