mod models;
mod normalize;
mod sources;
mod validate;

use normalize::normalize_indonesia_data;

use sources::indonesia::load_local_data;

use validate::validate_regions;

fn main() -> anyhow::Result<()> {
    let data = load_local_data()?;

    let regions = normalize_indonesia_data(data);

    validate_regions(&regions)
        .map_err(|errors| anyhow::anyhow!("validation failed:\n{}", errors.join("\n")))?;

    for region in regions {
        let parent = match region.parent_source_code {
            Some(parent_code) => parent_code,
            None => "None".to_string(),
        };

        println!(
            // "Country: {}, Source Code: {}, Name: {}, Level: {}, Type: {}, Parent Source Code: {}",
            "{} | {:<13} | {:<18} | level {} | {:<8} | parent: {}",
            region.country_code,
            region.source_code,
            region.name,
            region.level,
            region.region_type,
            parent
        );
    }

    Ok(())
}
