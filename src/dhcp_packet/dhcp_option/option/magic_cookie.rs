#[derive(Debug, Clone, PartialEq)]
pub struct MagicCookie;
        
impl MagicCookie {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
