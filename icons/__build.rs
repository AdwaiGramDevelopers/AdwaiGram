pub fn build(app_id: &str) {
    relm4_icons_build::bundle_icons(
        "icons.rs",
        Some(app_id),
        None::<&str>,
        Some("icons"),
        ["plus", "send-filled", "network-workgroup"],
    );
}
