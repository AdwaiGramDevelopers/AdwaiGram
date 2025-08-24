use AdwaiGram::constants::*;
use fluent::FluentResource;
use fluent::bundle::FluentBundle;
use intl_memoizer::IntlLangMemoizer;
use unic_langid::langid;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::os::unix;
use std::path::Path;

fn main() {
    // Try "en_US" and "ru_RU"
    // Borrow this in AppModel from Relm4
    // and add to Rc<>, please
    // =============== Fluent ===================
    let bundle = create_bundle("ru_RU");

    let msg = bundle
        .get_message("hello-world")
        .expect("Message doesn't exist.");
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(&pattern, None, &mut errors);
    // =============== Fluent ===================

    println!("{}", &value);

    println!("{VERSION}");
    println!("{TG_API_ID}");
    println!("{TG_API_HASH}");
}

fn create_bundle(lang: &str) -> FluentBundle<FluentResource, IntlLangMemoizer> {
    let path_str = format!("/home/mantis/Документы/projects/Desktop/AdwaiGram/langs/{lang}.ftl");
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

fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
