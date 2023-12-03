#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeritDumpFile;

impl MeritDumpFile {
    pub(crate) fn decode(_data: &[u8]) -> Option<crate::DhcpOption> {
        unimplemented!();
    }
}
