#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterRequestList;

impl ParameterRequestList {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
