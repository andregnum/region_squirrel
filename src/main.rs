mod models;
mod sources;

use crate::sources::indonesia::load_local_data;

fn main() -> anyhow::Result<()> {
    let data = load_local_data()?;

    println!("Loaded {} provinces", data.provinces.len());
    println!("Loaded {} regencies", data.regencies.len());
    println!("Loaded {} districts", data.districts.len());
    println!("Loaded {} villages", data.villages.len());

    Ok(())
}
