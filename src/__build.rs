//! Build script module for generating application constants.
//!
//! This module is responsible for setting constants in `src/constants.rs`.
//!
//! # Processed constants
//! - `VERSION`: Project version (with optional info depending on build profile)
//! - `TG_API_ID`: Telegram API identifier (with fallback value for development purposes)
//! - `TG_API_HASH`: Telegram API hash (with fallback value for development purposes)

use std::env::var;

/// Configures compile-time environment variables for constants generation.
///
/// # Processed environment variables
/// - `TG_API_ID`: Falls back to `17349` if not set
/// - `TG_API_HASH`: Falls back to `"344583e45741c457fe1862106095a5eb"` if not set
/// - `BUILD_PROFILE`: Automatically detected from cargo profile
///
/// # Panics
/// Will panic if `TG_API_ID` is set but cannot be parsed as a valid `u32`.
pub fn build() {
    let profile = var("PROFILE").unwrap();
    println!("cargo:rustc-env=PROFILE={}", profile);

    let tg_api_id = var("TG_API_ID").unwrap_or(17349.to_string());
    tg_api_id
        .parse::<u32>()
        .expect("TG_API_ID must be a valid u32");
    println!("cargo:rustc-env=TG_API_ID={}", tg_api_id);

    let tg_api_hash = var("TG_API_HASH").unwrap_or("344583e45741c457fe1862106095a5eb".to_string());
    println!("cargo:rustc-env=TG_API_HASH={}", tg_api_hash);

    println!("cargo:rerun-if-env-changed=TG_API_ID");
    println!("cargo:rerun-if-env-changed=TG_API_ID");
}
