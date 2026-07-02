use crate::export::{export_regions_to_csv, export_regions_to_json};
use crate::normalize::normalize_indonesia_data;
use crate::sources::RegionSource;
use crate::sources::indonesia::{LocalIndonesiaSource, list_indonesia_source_files};
use crate::validate::validate_regions;

pub fn normalize_indonesia() -> anyhow::Result<()> {
    let source = LocalIndonesiaSource;
    let data = source.load()?;

    let regions = normalize_indonesia_data(data);

    validate_regions(&regions)
        .map_err(|errors| anyhow::anyhow!("validation failed:\n{}", errors.join("\n")))?;

    export_regions_to_json(&regions, "output/indonesia/regions.json")?;
    export_regions_to_csv(&regions, "output/indonesia/regions.csv")?;

    println!("Exported {} regions", regions.len());
    println!("JSON: output/indonesia/regions.json");
    println!("CSV: output/indonesia/regions.csv");

    Ok(())
}

pub fn print_indonesia_sources() {
    println!("Indonesia Sources:");

    for source_file in list_indonesia_source_files() {
        println!(
            "- {} -> {} -> {}",
            source_file.name, source_file.url, source_file.cache_path
        );
    }
}
