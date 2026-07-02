use crate::export::{export_regions_to_csv, export_regions_to_json};
use crate::fetch::fetch_source_file;
use crate::normalize::normalize_indonesia_data;
use crate::sources::RegionSource;
use crate::sources::indonesia::{
    BPS_SOURCE_CONFIG, LocalIndonesiaSource, build_bps_province_source_file,
    list_indonesia_source_files, preview_bps_source_urls,
};
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
    println!("Indonesia BPS source config:");
    println!("Base URL: {}", BPS_SOURCE_CONFIG.base_url);
    println!("Alt Base URL: {}", BPS_SOURCE_CONFIG.base_url_alt);
    println!("Periode merge: {}", BPS_SOURCE_CONFIG.periode_merge);
    println!();

    println!("BPS URL previews:");

    for (name, url) in preview_bps_source_urls() {
        println!("- {} -> {}", name, url);
    }

    println!();
    println!("Legacy static source files:");

    for source_file in list_indonesia_source_files() {
        println!(
            "- {} -> {} -> {}",
            source_file.name, source_file.url, source_file.cache_path
        );
    }

    let bps_province_source = build_bps_province_source_file();

    println!();
    println!("BPS fetch target:");
    println!(
        "- {} -> {} -> {}",
        bps_province_source.name, bps_province_source.url, bps_province_source.cache_path
    );
}
pub fn fetch_indonesia_sources() -> anyhow::Result<()> {
    println!("Fetching Indonesia BPS province sources...");

    let source_file = build_bps_province_source_file();

    println!("Fetching {} from {}", source_file.name, source_file.url);

    fetch_source_file(&source_file)?;

    println!("Cached {} to {}", source_file.name, source_file.cache_path);

    Ok(())
}
