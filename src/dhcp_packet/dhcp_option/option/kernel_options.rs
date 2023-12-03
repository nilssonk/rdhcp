#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelOptions;

impl KernelOptions {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
