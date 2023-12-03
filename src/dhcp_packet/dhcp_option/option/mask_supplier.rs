#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaskSupplier;

impl MaskSupplier {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
