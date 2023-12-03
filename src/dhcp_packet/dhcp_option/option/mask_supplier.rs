#[derive(Debug, Clone, PartialEq)]
pub struct MaskSupplier;
        
impl MaskSupplier {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
