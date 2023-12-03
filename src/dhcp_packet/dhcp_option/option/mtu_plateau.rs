#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MtuPlateau;

impl MtuPlateau {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
