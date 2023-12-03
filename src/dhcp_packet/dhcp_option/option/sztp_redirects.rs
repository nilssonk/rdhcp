#[derive(Debug, Clone, PartialEq)]
pub struct SztpRedirects;
        
impl SztpRedirects {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
