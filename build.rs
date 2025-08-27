#[path = "data/__build.rs"]
mod data;
#[path = "src/__build.rs"]
mod src;
#[path = "locales/__build.rs"]
mod locales;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    data::build();
    src::build();
    locales::build();

    Ok(())
}
