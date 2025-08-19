use std::env::var;
use std::error::Error;
use std::fs::write;
use std::path::Path;

pub mod version {
    use super::*;
    pub fn build(profile: &str, app_id: &mut String, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut version: String = var("CARGO_PKG_VERSION").unwrap();

        match profile {
            "release" => (),
            _ => {
                let version_postfix = match std::process::Command::new("git")
                    .args(["rev-parse", "--short", "HEAD"])
                    .output()
                {
                    Ok(result) => String::from_utf8(result.stdout).unwrap(),
                    Err(_) => "devel".to_string(),
                };
                version += &format!("::{version_postfix}");
                *app_id += ".devel";
            }
        };
        write(out.join("version"), version.trim())?;
        write(out.join("app_id"), app_id)?;
        write(out.join("profile"), profile)?;
        Ok(())
    }
}

pub mod api {
    use super::*;

    pub fn build(out: &Path) -> Result<(), Box<dyn Error>> {
        let api_id = var("TG_API_ID").unwrap_or(17349.to_string());
        let api_hash = var("TG_API_HASH").unwrap_or("344583e45741c457fe1862106095a5eb".to_string());

        write(out.join("api_id"), api_id)?;
        write(out.join("api_hash"), api_hash)?;

        Ok(())
    }
}
