#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VendorClass;

impl VendorClass {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
