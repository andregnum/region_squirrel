use crate::{
    models::{RawDistrict, RawProvince, RawRegency, RawVillage, Region},
    sources::indonesia::IndonesiaLocalData,
};

pub fn normalize_provinces(provinces: Vec<RawProvince>) -> Vec<Region> {
    provinces
        .into_iter()
        .map(|province| Region {
            country_code: "ID".to_string(),
            source_code: province.code,
            name: province.name,
            level: 1,
            region_type: "province".to_string(),
            parent_source_code: None,
        })
        .collect()
}

pub fn normalize_regencies(regencies: Vec<RawRegency>) -> Vec<Region> {
    regencies
        .into_iter()
        .map(|regency| Region {
            country_code: "ID".to_string(),
            source_code: regency.code,
            name: regency.name,
            level: 2,
            region_type: "regency".to_string(),
            parent_source_code: Some(regency.province_code),
        })
        .collect()
}

pub fn normalize_districts(districts: Vec<RawDistrict>) -> Vec<Region> {
    districts
        .into_iter()
        .map(|district| Region {
            country_code: "ID".to_string(),
            source_code: district.code,
            name: district.name,
            level: 3,
            region_type: "district".to_string(),
            parent_source_code: Some(district.regency_code),
        })
        .collect()
}

pub fn normalize_villages(villages: Vec<RawVillage>) -> Vec<Region> {
    villages
        .into_iter()
        .map(|village| Region {
            country_code: "ID".to_string(),
            source_code: village.code,
            name: village.name,
            level: 4,
            region_type: "village".to_string(),
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
