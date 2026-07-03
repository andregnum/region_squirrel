use crate::export::{export_regions_to_csv, export_regions_to_json};
use crate::fetch::fetch_source_file;
use crate::normalize::{
    normalize_bps_provinces, normalize_bps_regencies, normalize_indonesia_data,
};
use crate::sources::RegionSource;
use crate::sources::indonesia::{
    BPS_SOURCE_CONFIG, LocalIndonesiaSource, build_bps_province_source_file,
    build_bps_regency_source_file, list_indonesia_source_files, load_cached_bps_provinces,
    load_cached_bps_regencies, preview_bps_source_urls,
};
use crate::validate::validate_regions;

const BPS_PROVINCES_JSON_OUTPUT_PATH: &str = "output/indonesia/bps/provinces.json";
const BPS_PROVINCES_CSV_OUTPUT_PATH: &str = "output/indonesia/bps/provinces.csv";
const BPS_REGIONS_JSON_OUTPUT_PATH: &str = "output/indonesia/bps/regions.json";
const BPS_REGIONS_CSV_OUTPUT_PATH: &str = "output/indonesia/bps/regions.csv";

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

    let province_source_file = build_bps_province_source_file();

    println!(
        "Fetching {} from {}",
        province_source_file.name, province_source_file.url
    );

    fetch_source_file(&province_source_file)?;

    println!(
        "Cached {} to {}",
        province_source_file.name, province_source_file.cache_path
    );

    let provinces = load_cached_bps_provinces()?;

    println!();
    println!(
        "Fetching BPS regency sources for {} provinces...",
        provinces.len()
    );

    for province in provinces {
        let source_file = build_bps_regency_source_file(&province.kode_bps);

        println!("Fetching {} from {}", source_file.name, source_file.url);

        fetch_source_file(&source_file)?;

        println!("Cached {} to {}", source_file.name, source_file.cache_path);
    }

    Ok(())
}
pub fn parse_bps_indonesia_sources() -> anyhow::Result<()> {
    let provinces = load_cached_bps_provinces()?;

    println!("Parsed {} BPS province records", provinces.len());

    for province in provinces.iter().take(10) {
        println!(
            "- {} | {} | {} | {}",
            province.kode_bps, province.nama_bps, province.kode_dagri, province.nama_dagri
        );
    }

    if provinces.len() > 10 {
        println!("... and {} more provinces", provinces.len() - 10);
    }

    let province_regions = normalize_bps_provinces(&provinces);

    validate_regions(&province_regions).map_err(|errors| {
        anyhow::anyhow!("BPS province validation failed:\n{}", errors.join("\n"))
    })?;

    export_regions_to_json(&province_regions, BPS_PROVINCES_JSON_OUTPUT_PATH)?;
    export_regions_to_csv(&province_regions, BPS_PROVINCES_CSV_OUTPUT_PATH)?;

    println!();
    println!(
        "Normalized {} BPS provinces to regions",
        province_regions.len()
    );
    println!("JSON: {}", BPS_PROVINCES_JSON_OUTPUT_PATH);
    println!("CSV: {}", BPS_PROVINCES_CSV_OUTPUT_PATH);

    for region in province_regions.iter().take(10) {
        let parent = region.parent_source_code.as_deref().unwrap_or("None");

        println!(
            "- {} | {} | {} | level {} | {} | parent: {}",
            region.country_code,
            region.source_code,
            region.name,
            region.level,
            region.region_type,
            parent
        );
    }

    if province_regions.len() > 10 {
        println!(
            "... and {} more province regions",
            province_regions.len() - 10
        );
    }

    let regencies = load_cached_bps_regencies()?;

    println!();
    println!("Parsed {} BPS regency records", regencies.len());

    for regency in regencies.iter().take(10) {
        println!(
            "- parent_bps={} | {} | {} | {} | {}",
            regency.parent_bps_code,
            regency.record.kode_bps,
            regency.record.nama_bps,
            regency.record.kode_dagri,
            regency.record.nama_dagri,
        );
    }

    if regencies.len() > 10 {
        println!("... and {} more regencies", regencies.len() - 10);
    }

    let regency_regions = normalize_bps_regencies(&provinces, regencies);

    let mut all_regions = Vec::new();
    all_regions.extend(province_regions);
    all_regions.extend(regency_regions);

    validate_regions(&all_regions).map_err(|errors| {
        anyhow::anyhow!(
            "BPS province/regency validation failed:\n{}",
            errors.join("\n")
        )
    })?;

    export_regions_to_json(&all_regions, BPS_REGIONS_JSON_OUTPUT_PATH)?;
    export_regions_to_csv(&all_regions, BPS_REGIONS_CSV_OUTPUT_PATH)?;

    println!();
    println!(
        "Normalized BPS province + regency regions: {}",
        all_regions.len()
    );

    println!("JSON: {}", BPS_REGIONS_JSON_OUTPUT_PATH);
    println!("CSV: {}", BPS_REGIONS_CSV_OUTPUT_PATH);

    for region in all_regions
        .iter()
        .filter(|region| region.level == 2)
        .take(10)
    {
        let parent = region.parent_source_code.as_deref().unwrap_or("None");

        println!(
            "- {} | {} | {} | level {} | {} | parent: {}",
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
