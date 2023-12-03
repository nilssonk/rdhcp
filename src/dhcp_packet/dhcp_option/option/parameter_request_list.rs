#[derive(Debug, Clone, PartialEq)]
pub struct ParameterRequestList;
        
impl ParameterRequestList {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
