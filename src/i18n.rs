use std::fs::File;
use std::io;
use std::io::Read as _;
use std::path::Path;

use fluent::bundle::FluentBundle;
use fluent::FluentResource;
use intl_memoizer::IntlLangMemoizer;
use unic_langid::langid;

pub fn create_bundle(lang: &str) -> FluentBundle<FluentResource, IntlLangMemoizer> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path_str = format!("{}/langs/{}.ftl", manifest_dir, lang);
    let path = Path::new(&path_str);
    let ftl_string = cat(path).unwrap();
    let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

    let langid_lg = match lang {
        "ru_RU" => langid!("ru-RU"),
        _ => langid!("en-US"),
    };
    let mut bundle = FluentBundle::new(vec![langid_lg]);

    bundle
        .add_resource(res)
        .expect("Failed to add FTL resources to the bundle.");

    bundle
}

pub fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
