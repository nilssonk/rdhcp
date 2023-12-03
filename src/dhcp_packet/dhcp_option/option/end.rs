#[derive(Debug, Clone, PartialEq, Eq)]
pub struct End;

impl End {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
