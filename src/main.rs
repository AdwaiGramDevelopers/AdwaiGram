use AdwaiGram::constants::*;
use AdwaiGram::i18n::create_bundle;

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

