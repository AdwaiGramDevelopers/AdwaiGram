use std::sync::LazyLock;

use i18n_embed::DefaultLocalizer;
use i18n_embed::DesktopLanguageRequester;
use i18n_embed::LanguageLoader;
use i18n_embed::Localizer as _;
use i18n_embed::fluent::FluentLanguageLoader;
use i18n_embed::fluent::fluent_language_loader;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "__locales_compiled/"]
struct Localizations;

pub static LOCALIZATIONS_LOADER: LazyLock<FluentLanguageLoader> = LazyLock::new(|| {
    let loader = fluent_language_loader!();

    loader
        .load_fallback_language(&Localizations)
        .expect("Failed to load embedded languages.");

    loader
});

fn localizer() -> DefaultLocalizer<'static> {
    DefaultLocalizer::new(&*LOCALIZATIONS_LOADER, &Localizations)
}

pub fn init() {
    let localizer = localizer();
    let requested_languages = DesktopLanguageRequester::requested_languages();

    if let Err(err) = localizer.select(&requested_languages) {
        eprintln!("Failed to set languages: {err}");
    }
}

#[macro_export]
macro_rules! tr {
    ($msg_id:literal) => {{
        i18n_embed_fl::fl!($crate::i18n::LOCALIZATIONS_LOADER, $msg_id)
    }};

     ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::i18n::LOCALIZATIONS_LOADER, $message_id, $($args), *)
    }};
}
