#[derive(Debug, Clone, PartialEq)]
pub struct MeritDumpFile;
        
impl MeritDumpFile {
    pub(crate) fn decode(data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
