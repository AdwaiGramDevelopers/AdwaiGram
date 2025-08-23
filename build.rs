#[path = "data/__build.rs"]
mod data;
#[path = "src/__build.rs"]
mod src;

use std::env::var;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    data::build();
    src::build();

    Ok(())
}
