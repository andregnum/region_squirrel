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

