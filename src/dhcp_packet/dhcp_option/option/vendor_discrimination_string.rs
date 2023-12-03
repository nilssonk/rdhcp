#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VendorDiscriminationString;

impl VendorDiscriminationString {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
