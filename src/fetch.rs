use std::thread::sleep;
use std::time::Duration;

use anyhow::Context;

use crate::cache::write_text_to_cache;
use crate::sources::SourceFile;

const DEFAULT_FETCH_DELAY_MS: u64 = 750;
const DEFAULT_RETRY_DELAY_MS: u64 = 2_000;
const MAX_FETCH_ATTEMPTS: usize = 3;

pub fn fetch_source_file(source_file: &SourceFile) -> anyhow::Result<()> {
    let mut last_error: Option<anyhow::Error> = None;

    for attempt in 1..=MAX_FETCH_ATTEMPTS {
        match fetch_source_file_once(source_file) {
            Ok(body) => {
                write_text_to_cache(&body, &source_file.cache_path)?;

                return Ok(());
            }
            Err(error) => {
                eprintln!(
                    "Fetch attempt {}/{} failed for {}: {}",
                    attempt, MAX_FETCH_ATTEMPTS, source_file.url, error
                );

                last_error = Some(error);

                if attempt < MAX_FETCH_ATTEMPTS {
                    wait_before_retry();
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| anyhow::anyhow!("failed to fetch {}", source_file.url)))
}

fn fetch_source_file_once(source_file: &SourceFile) -> anyhow::Result<String> {
    throttle_fetch();

    let response = reqwest::blocking::get(&source_file.url)
        .with_context(|| format!("failed to fetch {}", source_file.url))?;

    let response = response
        .error_for_status()
        .with_context(|| format!("source returned error status {}", source_file.url))?;

    let body = response
        .text()
        .with_context(|| format!("failed to read response body {}", source_file.url))?;

    Ok(body)
}

fn throttle_fetch() {
    sleep(Duration::from_millis(DEFAULT_FETCH_DELAY_MS));
}
fn wait_before_retry() {
    sleep(Duration::from_millis(DEFAULT_RETRY_DELAY_MS));
}
