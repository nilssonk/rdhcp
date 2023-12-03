#[derive(Debug, Clone, PartialEq)]
pub struct KernelOptions;
        
impl KernelOptions {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
