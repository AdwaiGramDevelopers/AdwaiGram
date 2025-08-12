fn main() {
    // i likes icons.toml so
    // TODO: read configuration from toml

    relm4_icons_build::bundle_icons(
        "icons.rs",
        None::<&str>,
        None::<&str>,
        Some("icons"),
        ["plus"],
    )
}
