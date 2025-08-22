#[path = "data/__build.rs"]
mod data;

use std::env::var;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    data::build();

    Ok(())
}
