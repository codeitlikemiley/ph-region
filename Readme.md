# PH Regions Rust Library

[![Rust](https://github.com/codeitlikemiley/ph-region/actions/workflows/rust.yml/badge.svg)](https://github.com/codeitlikemiley/ph-region/actions/workflows/rust.yml)

The `ph-region` Rust module provides a comprehensive way to manage and interact with a predefined set of regions, such as administrative regions in the Philippines. This module allows easy access to region details through various utility functions.

## Features

- **List Region Details**: You can list region codes, names, and abbreviations.
- **Parse from Strings**: Convert string input into region enum instances.
- **Display Full Names and Codes**: Easily retrieve full names or codes of regions for display purposes.
- **Key-Value Mapping**: Get a mapping of region codes to names or full names to codes.

## Installation

Add the following to your Cargo.toml file:

```toml
[dependencies]
ph-region = "0.1.0"
```

Replace "0.1.0" with the latest version of ph-region.


## Usage

### Listing Regions

You can list the keys, codes, or names of all regions:

```rust
use ph_region::region::Region;

fn main() {
    // Different way to List the Regions
    // list of keys
    println!("{:?}",Region::keys());
    // list of regions abbrev name
    println!("{:?}",Region::codes());
    // list of region name
    println!("{:?}",Region::names());
}
```

### Parsing and Displaying Region Details

```rust
use ph_region::region::Region;

fn main() {
    // Parse a region from a string and print it if valid
    if let Some(region) = Region::from_str("ncr") {
        println!("Region parsed: {:?}", region);
        println!("Region name: {}", region.name());
    }

    // Display the full name of a region from a numeric code
    if let Some(region) = Region::from_str("1") {
        println!("Full name: {}", region.full_name());
    }
}
```

### Key-Value Pairs

```rust
use ph_region::region::Region;

fn main() {
    // Display regions as code to name key-value pairs
    println!("{:?}", Region::list());

    // Display regions as name to code key-value pairs
    println!("{:?}", Region::list_by_full_name());
}
```

