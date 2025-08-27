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

pub const TG_API_ID: u32 = tg_api_id();
const fn tg_api_id() -> u32 {
    const_str::parse!(env!("TG_API_ID"), u32)
}

pub const TG_API_HASH: &str = env!("TG_API_HASH");

pub const APP_NAME: &str = app_name();
const fn app_name() -> &'static str {
    match env!("PROFILE").as_bytes() {
        b"release" => "AdwaiGram",
        &_ => "AdwaiGram (Devel)",
    }
}
