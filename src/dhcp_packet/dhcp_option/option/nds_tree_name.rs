#[derive(Debug, Clone, PartialEq)]
pub struct NdsTreeName;
        
impl NdsTreeName {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
