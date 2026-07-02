pub mod indonesia;
use crate::sources::indonesia::IndonesiaLocalData;

pub trait RegionSource {
    fn load(&self) -> anyhow::Result<IndonesiaLocalData>;
}
