use std::thread::sleep;
use std::time::Duration;

use anyhow::Context;

use crate::cache::write_text_to_cache;
use crate::sources::SourceFile;

const DEFAULT_FETCH_DELAY_MS: u64 = 750;

pub fn fetch_source_file(source_file: &SourceFile) -> anyhow::Result<()> {
    throttle_fetch();

    let response = reqwest::blocking::get(&source_file.url)
        .with_context(|| format!("failed to fetch {}", source_file.url))?;

    let response = response
        .error_for_status()
        .with_context(|| format!("source returned error status {}", source_file.url))?;

    let body = response
        .text()
        .with_context(|| format!("failed to read response body {}", source_file.url))?;

    write_text_to_cache(&body, &source_file.cache_path)?;

    Ok(())
}
fn throttle_fetch() {
    sleep(Duration::from_millis(DEFAULT_FETCH_DELAY_MS));
}
