#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RawProvince {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct RawRegency {
    pub code: String,
    pub province_code: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct RawDistrict {
    pub code: String,
    pub regency_code: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct RawVillage {
    pub code: String,
    pub district_code: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct Region {
    pub country_code: String,
    pub source_code: String,
    pub name: String,
    pub level: u8,
    pub region_type: String,
    pub parent_source_code: Option<String>,
}
