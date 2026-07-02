use std::fs::File;
use std::path::Path;

use anyhow::Context;

use crate::cache::copy_file_to_cache;
use crate::models::{RawDistrict, RawProvince, RawRegency, RawVillage};
use crate::sources::{BpsSourceConfig, RegionSource, SourceFile};

pub struct LocalIndonesiaSource;

pub const BPS_SOURCE_CONFIG: BpsSourceConfig = BpsSourceConfig {
    base_url: "https://sig.bps.go.id/rest-bridging/getwilayah",
    base_url_alt: "https://sig.bps.go.id/rest-bridging-dagri/getwilayah",
    periode_merge: "2025_1.2025",
};

#[derive(Debug, Clone, Copy)]
pub enum BpsRegionLevel {
    Province,
    Regency,
    District,
    Village,
}

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

impl BpsRegionLevel {
    pub fn as_query_value(&self) -> &'static str {
        match self {
            Self::Province => "provinsi",
            Self::Regency => "kabupaten",
            Self::District => "kecamatan",
            Self::Village => "desa",
        }
    }

    pub fn as_cache_name(&self) -> &'static str {
        match self {
            Self::Province => "provinces",
            Self::Regency => "regencies",
            Self::District => "districts",
            Self::Village => "villages",
        }
    }
}

impl RegionSource for LocalIndonesiaSource {
    fn load(&self) -> anyhow::Result<IndonesiaLocalData> {
        cache_local_raw_data()?;
        load_local_data()
    }
}
pub fn list_indonesia_source_files() -> Vec<SourceFile> {
    vec![
        SourceFile {
            name: "provinces".to_string(),
            url: "https://example.com/indonesia/provinces.json".to_string(),
            cache_path: "cache/raw/indonesia/provinces.json".to_string(),
        },
        SourceFile {
            name: "regencies".to_string(),
            url: "https://example.com/indonesia/regencies.json".to_string(),
            cache_path: "cache/raw/indonesia/regencies.json".to_string(),
        },
        SourceFile {
            name: "districts".to_string(),
            url: "https://example.com/indonesia/districts.json".to_string(),
            cache_path: "cache/raw/indonesia/districts.json".to_string(),
        },
        SourceFile {
            name: "villages".to_string(),
            url: "https://example.com/indonesia/villages.json".to_string(),
            cache_path: "cache/raw/indonesia/villages.json".to_string(),
        },
    ]
}
pub fn build_bps_source_url(level: BpsRegionLevel, parent: Option<&str>) -> String {
    let level_value = level.as_query_value();

    let mut url = format!("{}?level={}", BPS_SOURCE_CONFIG.base_url, level_value);

    if let Some(parent) = parent {
        url.push_str(&format!("&parent={}", parent));
    }

    url.push_str(&format!(
        "&periode_merge={}",
        BPS_SOURCE_CONFIG.periode_merge
    ));

    url
}
pub fn preview_bps_source_urls() -> Vec<(String, String)> {
    vec![
        (
            "provinces".to_string(),
            build_bps_source_url(BpsRegionLevel::Province, None),
        ),
        (
            "regencies example parent=21".to_string(),
            build_bps_source_url(BpsRegionLevel::Regency, Some("21")),
        ),
        (
            "districts example parent=2171".to_string(),
            build_bps_source_url(BpsRegionLevel::District, Some("2171")),
        ),
        (
            "villages example parent=2171010".to_string(),
            build_bps_source_url(BpsRegionLevel::Village, Some("2171010")),
        ),
    ]
}
pub fn build_bps_cache_path(level: BpsRegionLevel, parent: Option<&str>) -> String {
    match parent {
        Some(parent) => format!(
            "cache/raw/indonesia/{}/{}.json",
            level.as_cache_name(),
            parent
        ),
        None => format!("cache/raw/indonesia/{}.json", level.as_cache_name()),
    }
}
pub fn build_bps_province_source_file() -> SourceFile {
    SourceFile {
        name: "bps-provinces".to_string(),
        url: build_bps_source_url(BpsRegionLevel::Province, None),
        cache_path: build_bps_cache_path(BpsRegionLevel::Province, None),
    }
}
