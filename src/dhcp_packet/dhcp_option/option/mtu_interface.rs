#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MtuInterface;

impl MtuInterface {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
