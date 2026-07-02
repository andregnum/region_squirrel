pub mod indonesia;
use crate::sources::indonesia::IndonesiaLocalData;

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub name: &'static str,
    pub url: &'static str,
    pub cache_path: &'static str,
}
pub trait RegionSource {
    fn load(&self) -> anyhow::Result<IndonesiaLocalData>;
}
