use crate::models::Region;
use std::collections::HashSet;

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

fn validate_empty_fields(regions: &[Region], errors: &mut Vec<String>) {
    for region in regions {
        if region.country_code.trim().is_empty() {
            errors.push(format!(
                "region {} has empty country_code",
                region.source_code
            ));
        }

        if region.source_code.trim().is_empty() {
            errors.push("region has empty source_code".to_string());
        }

        if region.name.trim().is_empty() {
            errors.push(format!("region {} has empty name", region.source_code));
        }

        if region.region_type.trim().is_empty() {
            errors.push(format!(
                "region {} has empty region_type",
                region.source_code
            ));
        }
    }
}

fn validate_duplicate_codes(regions: &[Region], errors: &mut Vec<String>) {
    let mut seen_codes = HashSet::new();

    for region in regions {
        if !seen_codes.insert(region.source_code.as_str()) {
            errors.push(format!("duplicate source_code: {}", region.source_code));
        }
    }
}

fn validate_hierarchy(regions: &[Region], errors: &mut Vec<String>) {
    let codes: HashSet<&str> = regions
        .iter()
        .map(|region| region.source_code.as_str())
        .collect();

    for region in regions {
        match region.level {
            1 => {
                if region.parent_source_code.is_some() {
                    errors.push(format!(
                        "province {} must not have parent",
                        region.source_code
                    ));
                }
            }
            2 | 3 | 4 => {
                let Some(parent_code) = &region.parent_source_code else {
                    errors.push(format!(
                        "region {} level {} must have parent",
                        region.source_code, region.level
                    ));
                    continue;
                };

                if !codes.contains(parent_code.as_str()) {
                    errors.push(format!(
                        "region {} references missing parent {}",
                        region.source_code, parent_code
                    ));
                }
            }
            _ => {
                errors.push(format!(
                    "region {} has invalid level {}",
                    region.source_code, region.level
                ));
            }
        }
    }
}
