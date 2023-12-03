#[derive(Debug, Clone, PartialEq)]
pub struct MaxDatagramAssembly;
        
impl MaxDatagramAssembly {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
