pub fn build(app_id: &str) {
    glib_build_tools::compile_resources(
        &["data"],
        "data/resources.gresource.xml",
        "compiled.gresource",
    );
    relm4_icons_build::bundle_icons(
        "icons.rs",
        Some(app_id),
        None,
        Some("data/icons"),
        Vec::from([
            "send-filled"
        ]),
    );
}
