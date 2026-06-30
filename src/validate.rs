use std::collections::{HashMap, HashSet};
use crate::models::Region;

pub fn validate_regions(regions: &[Region]) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    validate_empty_fields(regions, &mut errors);
    validate_duplicate_codes(regions, &mut errors);
    validate_hierarchy(regions, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

