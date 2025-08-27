use std::env::var;

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
