//! Application configuration constants.
//!
//! This module contains compile-time constants used for basic application configuration,
//! including various info like
//! Telegram API credentials, version information, app ID, etc.
//!
//! ## Note
//! Most constants are set via environment variables during compilation with fallback values
//! for development purposes. For normal use, ensure proper environment variables are set.
//!

/// Project version
/// computed at compile time, depends on the build profile and Git availability.
///
/// - **Release** builds (`--release`): Use the exact version from `Cargo.toml`
/// - **Debug** builds: Append Git commit information when available
///
/// # Examples
///
/// ## Release version
/// ```
/// pub const VERSION: &str = "0.1.0";
/// ```
///
/// ## Development versions with Git
/// ### Clean working tree
/// ```
/// pub const VERSION: &str = "0.1.0-1a2b3c4";
/// ```
/// ### Uncommitted changes present
/// ```
/// pub const VERSION: &str = "0.1.0-1a2b3c4-dirty";
/// ```
/// ### Git unavailable
/// ```
/// pub const VERSION: &str = "0.1.0-devel";
/// ```
pub const VERSION: &str = version();
const fn version() -> &'static str {
    match env!("PROFILE").as_bytes() {
        b"release" => env!("CARGO_PKG_VERSION"),
        &_ => git_version::git_version!(
            args = ["--always", "--dirty=-dirty"],
            prefix = concat!(env!("CARGO_PKG_VERSION"), "-"),
            fallback = concat!(env!("CARGO_PKG_VERSION"), "-devel")
        ),
    }
}

/// Telegram API ID used for client authentication.
///
/// This unique identifier is required by Telegram API to track your application usage.
/// The value is obtained from the environment variable `TG_API_ID` at compile time,
/// and fallback to **17349** if not present.
/// Must be set together with `TG_API_HASH`.
///
/// Fallback value is suitable for **development and testing purposes only**!
///
/// # Usage
/// Set the environment variables before building:
/// ```bash
/// export TG_API_ID=1234567
/// export TG_API_HASH=e017693e4a04a59d0b0f400fe98177fe
/// cargo build
/// ```
/// Or set it inline:
/// ```bash
/// TG_API_ID=1234567 TG_API_HASH=e017693e4a04a59d0b0f400fe98177fe cargo build
/// ```
pub const TG_API_ID: u32 = tg_api_id();
const fn tg_api_id() -> u32 {
    const_str::parse!(env!("TG_API_ID"), u32)
}

/// Telegram API hash used for client authentication.
///
/// This secret hash is required by Telegram API together with `TG_API_ID`
/// to authenticate your application. The value is obtained from the
/// environment variable `TG_API_HASH` at compile time,
/// and fallback to **344583e45741c457fe1862106095a5eb** if variable is not present.
///
/// Fallback value is suitable for **development and testing purposes only**!
///
/// # Usage
/// Set the environment variable before building together with `TG_API_ID`:
/// ```bash
/// export TG_API_ID=1234567
/// export TG_API_HASH="e017693e4a04a59d0b0f400fe98177fe"
/// cargo build
/// ```
///
/// Or set it inline for one-time compilation:
/// ```bash
/// TG_API_ID=1234567 TG_API_HASH="e017693e4a04a59d0b0f400fe98177fe" cargo build
/// ```
pub const TG_API_HASH: &str = env!("TG_API_HASH");
