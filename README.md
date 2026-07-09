# `region_squirrel`

`region_squirrel` is a Rust-based administrative region data crawler and normalizer.

The current MVP focuses on Indonesian administrative region data from BPS. It fetches source data, parses cached files, normalizes records into a canonical region model, validates hierarchy integrity, detects mapping conflicts, and exports the resulting datasets to JSON and CSV.

> **Status:** Alpha MVP  
> **Current focus:** Indonesia BPS administrative region data  
> **Intended use:** Data generation pipeline, not an application-level region library

## Why this project exists

Administrative region data is commonly required by business applications, reporting systems, address modules, and internal operational tools.

Rather than combining crawling, parsing, normalization, and application-specific logic into a single codebase, `region_squirrel` focuses exclusively on producing clean, canonical region datasets.

Application-facing libraries and integrations should be implemented in separate projects, such as a future `region-kit`.

## Features

- Fetch Indonesia BPS administrative region source data.
- Parse cached BPS source files.
- Normalize province, regency/city, district, and village records.
- Validate administrative hierarchy integrity.
- Detect conflicting BPS-DAGRI district mappings.
- Detect conflicting BPS-DAGRI village mappings.
- Export conflict reports to JSON and CSV.
- Export canonical region data to JSON and CSV.

## Current Limitations

This project is currently in the alpha stage.

Known limitations include:

- Only Indonesia is supported.
- Only the BPS data source is implemented.
- Conflict detection and matching heuristics still require additional refinement.
- Output schemas may evolve before the first stable release.
- This project is not intended to be consumed directly as a Node.js, NestJS, Prisma, or application library.

## Requirements

- Rust
- Cargo
- Internet connection (for fetching source data)

## Installation

Clone the repository:

```bash
git clone https://github.com/andregnum/region_squirrel.git
cd region_squirrel
```

Build the project:

```bash
cargo build
```

Show the available CLI commands:

```bash
cargo run -- --help
```

## CLI Usage

Show the configured Indonesia source:

```bash
cargo run -- sources indonesia
```

Fetch all available Indonesia BPS source data:

```bash
cargo run -- fetch indonesia all
```

Fetch a specific administrative level:

```bash
cargo run -- fetch indonesia provinces
cargo run -- fetch indonesia regencies
cargo run -- fetch indonesia districts
cargo run -- fetch indonesia villages
```

Parse, normalize, validate, and export canonical BPS data:

```bash
cargo run -- parse-bps indonesia
```

## Output

Canonical region dataset:

```text
output/indonesia/bps/regions.json
output/indonesia/bps/regions.csv
```

Province-only dataset:

```text
output/indonesia/bps/provinces.json
output/indonesia/bps/provinces.csv
```

Conflict reports:

```text
output/indonesia/bps/conflicts/districts.json
output/indonesia/bps/conflicts/districts.csv
output/indonesia/bps/conflicts/villages.json
output/indonesia/bps/conflicts/villages.csv
```

## Canonical Region Model

The normalized region output currently uses the following structure:

```json
{
  "country_code": "ID",
  "source_code": "32.04.12.2001",
  "name": "Example Village",
  "level": 4,
  "region_type": "village",
  "parent_source_code": "32.04.12"
}
```

Administrative levels:

| Level | Description |
|------:|-------------|
| 1 | Province |
| 2 | Regency / City |
| 3 | District |
| 4 | Village |

## Recommended Workflow

Run the pipeline in the following order:

```bash
cargo run -- sources indonesia
cargo run -- fetch indonesia all
cargo run -- parse-bps indonesia
```

Generated datasets and reports will be available under:

```text
output/indonesia/bps/
```

## Project Scope

`region_squirrel` is a data pipeline project.

Its responsibilities include:

- Crawling source data
- Caching raw source files
- Parsing
- Normalization
- Validation
- Conflict reporting
- Exporting canonical datasets

The project intentionally does **not** aim to become:

- A NestJS module
- A Prisma library
- An application-level region service
- A general-purpose address management library

Those responsibilities should be implemented in separate downstream projects.

## Roadmap

The current milestone is:

**v0.1.0-alpha.1**

This release represents the first end-to-end alpha MVP capable of generating canonical Indonesian administrative region datasets from BPS source data.

## License

TBD

If it ain't broke, don't fix it.
