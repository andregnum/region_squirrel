pub mod bps;
pub mod local;

pub use bps::{
    BPS_SOURCE_CONFIG, BpsDistrictRecord, BpsRegencyRecord, BpsRegionRecord, BpsVillageRecord,
    build_bps_district_source_file, build_bps_province_source_file, build_bps_regency_source_file,
    build_bps_village_source_file, load_cached_bps_districts, load_cached_bps_provinces,
    load_cached_bps_regencies, load_cached_bps_villages, preview_bps_source_urls,
};

pub use local::{IndonesiaLocalData, LocalIndonesiaSource};

use crate::sources::SourceFile;

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
