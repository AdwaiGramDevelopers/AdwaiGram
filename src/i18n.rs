//! # Internationalization (i18n)
//! This module provides comprehensive internationalization support for loading and managing
//! localized resources at runtime using `rust-embed` and `fluent`.
//!
//! Localization files are being embedded during compilation using `rust-embed` and provides
//! runtime language selection based on the user's desktop environment preferences.

use std::sync::LazyLock;

use i18n_embed::DefaultLocalizer;
use i18n_embed::DesktopLanguageRequester;
use i18n_embed::LanguageLoader;
use i18n_embed::Localizer as _;
use i18n_embed::fluent::FluentLanguageLoader;
use i18n_embed::fluent::fluent_language_loader;
use rust_embed::RustEmbed;

/// A RustEmbed structure that embeds all localization files from the directory set in `folder` attribute
/// into the binary during compilation. This allows the application to access translation files without
/// requiring external file system access at runtime.
#[derive(RustEmbed)]
#[folder = "locales/"]
struct Localizations;

/// Lazily-initialized static instance of `FluentLanguageLoader`
/// that loads the fallback language at startup.
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

/// Initializes the internationalization system by detecting the user's preferred languages
/// from the desktop environment and configuring the localizer accordingly.
/// This function should be called early in the application startup process.
pub fn init() {
    let localizer = localizer();
    let requested_languages = DesktopLanguageRequester::requested_languages();

    if let Err(err) = localizer.select(&requested_languages) {
        eprintln!("Failed to set languages: {err}");
    }
}

/// Get translated message by key, with optional translation parameters
///
/// # Examples:
///
/// Without parameters:
///
/// ```ignore
/// println!("Translated message: {}", tr!("developers"));
/// ```
///
/// With parameters:
///
/// ```ignore
/// println!("Translated message: {}", tr!("last-seen", {
///     "time" = "5 minutes ago"
/// }));
/// ```
#[macro_export]
macro_rules! tr {
    ($msg_id:literal) => {{
        i18n_embed_fl::fl!($crate::i18n::LOCALIZATIONS_LOADER, $msg_id)
    }};

     ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::i18n::LOCALIZATIONS_LOADER, $message_id, $($args), *)
    }};
}
