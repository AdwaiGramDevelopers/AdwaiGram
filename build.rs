#[path = "data/__build.rs"]
mod data;
#[path = "icons/__build.rs"]
mod icons;
#[path = "src/__build.rs"]
mod src;

use std::env::var;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir: std::path::PathBuf = std::path::PathBuf::from(var("OUT_DIR").unwrap());
    let mut app_id: String = "app.AdwaiGramDevelopers.AdwaiGram".to_string();
    let profile: String = var("PROFILE").unwrap();

    src::version::build(&profile, &mut app_id, &out_dir)?;
    src::api::build(&out_dir)?;

    icons::build(&app_id);
    data::build();

    Ok(())
}
