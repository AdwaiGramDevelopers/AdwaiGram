#[path = "data/__build.rs"]
mod data;
#[path = "src/__build.rs"]
mod src;

use std::env::var;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(var("OUT_DIR").unwrap());
    let mut app_id = "app.adwaigramdevs.adwaigram".to_string();
    let profile = var("PROFILE").unwrap();

    src::version::build(&profile, &mut app_id, &out_dir)?;
    src::api::build(&out_dir)?;

    data::build(&app_id);

    Ok(())
}
