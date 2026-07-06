use std::collections::{HashMap, HashSet};

use crate::{
    models::{RawDistrict, RawProvince, RawRegency, RawVillage, Region},
    sources::indonesia::{
        BpsDistrictRecord, BpsRegencyRecord, BpsRegionRecord, IndonesiaLocalData,
    },
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
    regencies: &[BpsRegencyRecord],
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
                .unwrap_or_else(|| regency.parent_bps_code.clone());

            Region {
                country_code: INDONESIA_COUNTRY_CODE.to_string(),
                source_code: regency.record.kode_dagri.clone(),
                name: regency.record.nama_dagri.clone(),
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

#[derive(Debug, serde::Serialize)]
pub struct BpsDistrictConflict {
    pub parent_bps_code: String,
    pub kode_bps: String,
    pub nama_bps: String,
    pub kode_dagri: String,
    pub nama_dagri: String,
    pub reason: String,
}

pub fn normalize_bps_districts(
    regencies: &[BpsRegencyRecord],
    districts: Vec<BpsDistrictRecord>,
) -> Vec<Region> {
    let regency_dagri_by_bps: HashMap<&str, &str> = regencies
        .iter()
        .map(|regency| {
            (
                regency.record.kode_bps.as_str(),
                regency.record.kode_dagri.as_str(),
            )
        })
        .collect();

    districts
        .into_iter()
        .map(|district| {
            let parent_source_code = regency_dagri_by_bps
                .get(district.parent_bps_code.as_str())
                .map(|parent_dagri_code| (*parent_dagri_code).to_string())
                .unwrap_or(district.parent_bps_code);

            Region {
                country_code: INDONESIA_COUNTRY_CODE.to_string(),
                source_code: district.record.kode_dagri,
                name: district.record.nama_dagri,
                level: LEVEL_DISTRICT,
                region_type: TYPE_DISTRICT.to_string(),
                parent_source_code: Some(parent_source_code),
            }
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

fn find_conflicted_bps_district_keys(districts: &[BpsDistrictRecord]) -> HashSet<(String, String)> {
    let mut dagri_codes_by_bps_code: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut dagri_names_by_bps_code: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut bps_codes_by_dagri_code: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut bps_names_by_dagri_code: HashMap<&str, HashSet<&str>> = HashMap::new();

    for district in districts {
        dagri_codes_by_bps_code
            .entry(district.record.kode_bps.as_str())
            .or_default()
            .insert(district.record.kode_dagri.as_str());

        dagri_names_by_bps_code
            .entry(district.record.kode_bps.as_str())
            .or_default()
            .insert(district.record.nama_dagri.as_str());

        bps_codes_by_dagri_code
            .entry(district.record.kode_dagri.as_str())
            .or_default()
            .insert(district.record.kode_bps.as_str());

        bps_names_by_dagri_code
            .entry(district.record.kode_dagri.as_str())
            .or_default()
            .insert(district.record.nama_bps.as_str());
    }

    let mut conflicted_keys = HashSet::new();

    for district in districts {
        let kode_bps = district.record.kode_bps.as_str();
        let kode_dagri = district.record.kode_dagri.as_str();

        let bps_maps_to_multiple_dagri_codes = dagri_codes_by_bps_code
            .get(kode_bps)
            .is_some_and(|values| values.len() > 1);

        let bps_maps_to_multiple_dagri_names = dagri_names_by_bps_code
            .get(kode_bps)
            .is_some_and(|values| values.len() > 1);

        let dagri_maps_to_multiple_bps_codes = bps_codes_by_dagri_code
            .get(kode_dagri)
            .is_some_and(|values| values.len() > 1);

        let dagri_maps_to_multiple_bps_names = bps_names_by_dagri_code
            .get(kode_dagri)
            .is_some_and(|values| values.len() > 1);

        if bps_maps_to_multiple_dagri_codes
            || bps_maps_to_multiple_dagri_names
            || dagri_maps_to_multiple_bps_codes
            || dagri_maps_to_multiple_bps_names
        {
            conflicted_keys.insert((
                district.parent_bps_code.clone(),
                district.record.kode_bps.clone(),
            ));
        }
    }

    conflicted_keys
}
pub fn split_clean_bps_districts(
    districts: Vec<BpsDistrictRecord>,
) -> (Vec<BpsDistrictRecord>, Vec<BpsDistrictConflict>) {
    let conflicted_keys = find_conflicted_bps_district_keys(&districts);

    let mut clean_districts = Vec::new();
    let mut conflicts = Vec::new();

    for district in districts {
        let key = (
            district.parent_bps_code.clone(),
            district.record.kode_bps.clone(),
        );

        if conflicted_keys.contains(&key) {
            conflicts.push(BpsDistrictConflict {
                parent_bps_code: district.parent_bps_code,
                kode_bps: district.record.kode_bps,
                nama_bps: district.record.nama_bps,
                kode_dagri: district.record.kode_dagri,
                nama_dagri: district.record.nama_dagri,
                reason: "conflicting BPS-DAGRI district mapping".to_string(),
            });
        } else {
            clean_districts.push(district);
        }
    }

    (clean_districts, conflicts)
}
