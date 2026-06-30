// use serde::Deserialize;

#![allow(dead_code)]

#[derive(Debug, serde::Deserialize)]
pub struct RawProvince {
    pub code: String,
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawRegency {
    pub code: String,
    pub province_code: String,
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawDistrict {
    pub code: String,
    pub regencies_code: String,
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawVillage {
    pub code: String,
    pub district_code: String, 
    pub name: String,
}

#[derive(Debug)]
pub struct Region {
    pub country_code: String,
    pub source_code: String,
    pub name: String,
    pub level: u8,
    pub region_type: String,
    pub parent_source_code: Option<String>,
}