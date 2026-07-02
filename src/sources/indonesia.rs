use std::fs::File;
use std::path::Path;

use anyhow::Context;

use crate::cache::copy_file_to_cache;
use crate::models::{RawDistrict, RawProvince, RawRegency, RawVillage};
use crate::sources::RegionSource;

pub struct LocalIndonesiaSource;
#[derive(Debug)]
pub struct IndonesiaLocalData {
    pub provinces: Vec<RawProvince>,
    pub regencies: Vec<RawRegency>,
    pub districts: Vec<RawDistrict>,
    pub villages: Vec<RawVillage>,
}

pub fn load_local_data() -> anyhow::Result<IndonesiaLocalData> {
    let base_path = Path::new("fixtures/indonesia");

    let provinces: Vec<RawProvince> = read_json_file(base_path.join("provinces.json"))?;

    let regencies: Vec<RawRegency> = read_json_file(base_path.join("regencies.json"))?;

    let districts: Vec<RawDistrict> = read_json_file(base_path.join("districts.json"))?;

    let villages: Vec<RawVillage> = read_json_file(base_path.join("villages.json"))?;

    Ok(IndonesiaLocalData {
        provinces,
        regencies,
        districts,
        villages,
    })
}

pub fn cache_local_raw_data() -> anyhow::Result<()> {
    let base_path = Path::new("fixtures/indonesia");
    let cache_path = Path::new("cache/raw/indonesia");

    copy_file_to_cache(
        base_path.join("provinces.json"),
        cache_path.join("provinces.json"),
    )?;

    copy_file_to_cache(
        base_path.join("regencies.json"),
        cache_path.join("regencies.json"),
    )?;

    copy_file_to_cache(
        base_path.join("districts.json"),
        cache_path.join("districts.json"),
    )?;

    copy_file_to_cache(
        base_path.join("villages.json"),
        cache_path.join("villages.json"),
    )?;

    Ok(())
}

fn read_json_file<T>(path: impl AsRef<Path>) -> anyhow::Result<Vec<T>>
where
    T: serde::de::DeserializeOwned,
{
    let path = path.as_ref();

    let file = File::open(path).with_context(|| format!("failed to open {}", path.display()))?;

    let data = serde_json::from_reader(file)
        .with_context(|| format!("failed to parse {}", path.display()))?;

    Ok(data)
}

impl RegionSource for LocalIndonesiaSource {
    fn load(&self) -> anyhow::Result<IndonesiaLocalData> {
        cache_local_raw_data()?;
        load_local_data()
    }
}
