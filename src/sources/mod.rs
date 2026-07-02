pub mod indonesia;
use crate::sources::indonesia::IndonesiaLocalData;

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub name: &'static str,
    pub url: &'static str,
    pub cache_path: &'static str,
}
#[derive(Debug, Clone)]
pub struct BpsSourceConfig {
    pub base_url: &'static str,
    pub base_url_alt: &'static str,
    pub periode_merge: &'static str,
}
pub trait RegionSource {
    fn load(&self) -> anyhow::Result<IndonesiaLocalData>;
}
