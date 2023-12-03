#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagicCookie;

impl MagicCookie {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
