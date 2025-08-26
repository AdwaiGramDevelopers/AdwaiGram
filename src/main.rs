use AdwaiGram::constants::*;
use AdwaiGram::tr;
fn main() {
    AdwaiGram::i18n::init();
    println!("{}", tr!("hello-world"));

    println!("{VERSION}");
    println!("{TG_API_ID}");
    println!("{TG_API_HASH}");
}

