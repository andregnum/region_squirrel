use std::fs;
use std::path::Path;

use anyhow::Context;

pub fn copy_file_to_cache(
    source_path: impl AsRef<Path>,
    cache_path: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let source_path = source_path.as_ref();
    let cache_path = cache_path.as_ref();

    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create cache directory {}", parent.display()))?;
    }

    fs::copy(source_path, cache_path).with_context(|| {
        format!(
            "failed to copy {} to {}",
            source_path.display(),
            cache_path.display()
        )
    })?;

    Ok(())
}
pub fn write_text_to_cache(content: &str, cache_path: impl AsRef<Path>) -> anyhow::Result<()> {
    let cache_path = cache_path.as_ref();

    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create cache directory {}", parent.display()))?;
    }

    fs::write(cache_path, content)
        .with_context(|| format!("failed to write cache file {}", cache_path.display()))?;

    Ok(())
}
