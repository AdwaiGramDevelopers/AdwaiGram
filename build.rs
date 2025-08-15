use glib_build_tools;
use relm4_icons_build;

fn main() {
    relm4_icons_build::bundle_icons(
        "icon_names.rs",
        Some("app.adwaigramdevs.adwaigram"),
        None::<&str>,
        None::<&str>,
        ["send-filled", "plus"],
    );

    glib_build_tools::compile_resources(
        &["data"],
        "data/resources.gresource.xml",
        "compiled.gresource",
    );
}
