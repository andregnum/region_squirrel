mod models;
mod normalize;
mod sources;

use normalize::{
    normalize_districts, normalize_provinces, normalize_regencies, normalize_villages,
};

use sources::indonesia::load_local_data;

fn main() -> anyhow::Result<()> {
    let data = load_local_data()?;

    let mut regions = Vec::new();

    let provinces = normalize_provinces(data.provinces);
    let regencies = normalize_regencies(data.regencies);
    let districts = normalize_districts(data.districts);
    let villages = normalize_villages(data.villages);

    regions.extend(provinces);
    regions.extend(regencies);
    regions.extend(districts);
    regions.extend(villages);

    for region in regions {
        println!(
            "Country: {}, Source Code: {}, Name: {}, Level: {}, Type: {}, Parent Source Code: {:?}",
            region.country_code,
            region.source_code,
            region.name,
            region.level,
            region.region_type,
            region.parent_source_code
        );
    }

    Ok(())
}
