#[derive(Debug, Clone, PartialEq)]
pub struct End;
        
impl End {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
