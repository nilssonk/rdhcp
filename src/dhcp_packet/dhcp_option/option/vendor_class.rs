#[derive(Debug, Clone, PartialEq)]
pub struct VendorClass;
        
impl VendorClass {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
