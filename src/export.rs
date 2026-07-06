use std::fs::{self, File};
use std::path::Path;

use anyhow::Context;

use crate::models::Region;

pub fn export_regions_to_json(
    regions: &[Region],
    output_path: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let output_path = output_path.as_ref();

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create output directory {}", parent.display()))?;
    }

    let file = File::create(output_path)
        .with_context(|| format!("failed to create output file {}", output_path.display()))?;

    serde_json::to_writer_pretty(file, regions)
        .with_context(|| format!("failed to write JSON output {}", output_path.display()))?;

    Ok(())
}
pub fn export_regions_to_csv(
    regions: &[Region],
    output_path: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let output_path = output_path.as_ref();

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create output directory {}", parent.display()))?;
    }

    let mut writer = csv::Writer::from_path(output_path)
        .with_context(|| format!("failed to create CSV output {}", output_path.display()))?;

    for region in regions {
        writer
            .serialize(region)
            .with_context(|| format!("failed to serialize region {}", region.source_code))?;
    }

    writer
        .flush()
        .with_context(|| format!("failed to flush CSV output {}", output_path.display()))?;

    Ok(())
}
pub fn export_to_json<T>(data: &[T], output_path: impl AsRef<Path>) -> anyhow::Result<()>
where
    T: serde::Serialize,
{
    let output_path = output_path.as_ref();

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create output directory {}", parent.display()))?;
    }

    let file = File::create(output_path)
        .with_context(|| format!("failed to create output file {}", output_path.display()))?;

    serde_json::to_writer_pretty(file, data)
        .with_context(|| format!("failed to write JSON output {}", output_path.display()))?;

    Ok(())
}

pub fn export_to_csv<T>(data: &[T], output_path: impl AsRef<Path>) -> anyhow::Result<()>
where
    T: serde::Serialize,
{
    let output_path = output_path.as_ref();

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create output directory {}", parent.display()))?;
    }

    let mut writer = csv::Writer::from_path(output_path)
        .with_context(|| format!("failed to create CSV output {}", output_path.display()))?;

    for item in data {
        writer
            .serialize(item)
            .with_context(|| format!("failed to serialize CSV ro to {}", output_path.display()))?;
    }

    writer
        .flush()
        .with_context(|| format!("failed to flush CSV output {}", output_path.display()))?;

    Ok(())
}
