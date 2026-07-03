use std::collections::HashMap;

use crate::{
    models::{RawDistrict, RawProvince, RawRegency, RawVillage, Region},
    sources::indonesia::{BpsRegencyRecord, BpsRegionRecord, IndonesiaLocalData},
};

const INDONESIA_COUNTRY_CODE: &str = "ID";

const LEVEL_PROVINCE: u8 = 1;
const LEVEL_REGENCY: u8 = 2;
const LEVEL_DISTRICT: u8 = 3;
const LEVEL_VILLAGE: u8 = 4;

const TYPE_PROVINCE: &str = "province";
const TYPE_REGENCY: &str = "regency";
const TYPE_DISTRICT: &str = "district";
const TYPE_VILLAGE: &str = "village";

pub fn normalize_provinces(provinces: Vec<RawProvince>) -> Vec<Region> {
    provinces
        .into_iter()
        .map(|province| Region {
            country_code: INDONESIA_COUNTRY_CODE.to_string(),
            source_code: province.code,
            name: province.name,
            level: LEVEL_PROVINCE,
            region_type: TYPE_PROVINCE.to_string(),
            parent_source_code: None,
        })
        .collect()
}

pub fn normalize_bps_provinces(records: &[BpsRegionRecord]) -> Vec<Region> {
    records
        .into_iter()
        .map(|record| Region {
            country_code: INDONESIA_COUNTRY_CODE.to_string(),
            source_code: record.kode_dagri.clone(),
            name: record.nama_dagri.clone(),
            level: LEVEL_PROVINCE,
            region_type: TYPE_PROVINCE.to_string(),
            parent_source_code: None,
        })
        .collect()
}

pub fn normalize_regencies(regencies: Vec<RawRegency>) -> Vec<Region> {
    regencies
        .into_iter()
        .map(|regency| Region {
            country_code: INDONESIA_COUNTRY_CODE.to_string(),
            source_code: regency.code,
            name: regency.name,
            level: LEVEL_REGENCY,
            region_type: TYPE_REGENCY.to_string(),
            parent_source_code: Some(regency.province_code),
        })
        .collect()
}

pub fn normalize_bps_regencies(
    provinces: &[BpsRegionRecord],
    regencies: Vec<BpsRegencyRecord>,
) -> Vec<Region> {
    let province_dagri_by_bps: HashMap<&str, &str> = provinces
        .iter()
        .map(|province| (province.kode_bps.as_str(), province.kode_dagri.as_str()))
        .collect();

    regencies
        .into_iter()
        .map(|regency| {
            let parent_source_code = province_dagri_by_bps
                .get(regency.parent_bps_code.as_str())
                .map(|parent_dagri_code| (*parent_dagri_code).to_string())
                .unwrap_or(regency.parent_bps_code);

            Region {
                country_code: INDONESIA_COUNTRY_CODE.to_string(),
                source_code: regency.record.kode_dagri,
                name: regency.record.nama_dagri,
                level: LEVEL_REGENCY,
                region_type: TYPE_REGENCY.to_string(),
                parent_source_code: Some(parent_source_code),
            }
        })
        .collect()
}

pub fn normalize_districts(districts: Vec<RawDistrict>) -> Vec<Region> {
    districts
        .into_iter()
        .map(|district| Region {
            country_code: INDONESIA_COUNTRY_CODE.to_string(),
            source_code: district.code,
            name: district.name,
            level: LEVEL_DISTRICT,
            region_type: TYPE_DISTRICT.to_string(),
            parent_source_code: Some(district.regency_code),
        })
        .collect()
}

pub fn normalize_villages(villages: Vec<RawVillage>) -> Vec<Region> {
    villages
        .into_iter()
        .map(|village| Region {
            country_code: INDONESIA_COUNTRY_CODE.to_string(),
            source_code: village.code,
            name: village.name,
            level: LEVEL_VILLAGE,
            region_type: TYPE_VILLAGE.to_string(),
            parent_source_code: Some(village.district_code),
        })
        .collect()
}

pub fn normalize_indonesia_data(data: IndonesiaLocalData) -> Vec<Region> {
    let mut regions = Vec::new();

    regions.extend(normalize_provinces(data.provinces));
    regions.extend(normalize_regencies(data.regencies));
    regions.extend(normalize_districts(data.districts));
    regions.extend(normalize_villages(data.villages));

    regions
}
