#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DotsRi;

impl DotsRi {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
