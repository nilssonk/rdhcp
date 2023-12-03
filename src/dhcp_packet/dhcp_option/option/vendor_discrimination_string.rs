#[derive(Debug, Clone, PartialEq)]
pub struct VendorDiscriminationString;
        
impl VendorDiscriminationString {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
