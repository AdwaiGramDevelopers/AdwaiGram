use relm4::adw::prelude::AdwDialogExt as _;
use relm4::prelude::*;

use crate::constants::PROFILE;

pub struct AboutPage;

#[relm4::component(pub)]
impl SimpleComponent for AboutPage {
    type Init = adw::Dialog;
    type Input = ();
    type Output = ();

    view! {
        adw::AboutDialog {
            set_application_icon: match PROFILE {
                "release" => crate::ui::icons::custom::logo::MAIN,
                _ => crate::ui::icons::custom::logo::DEVEL,
            },
            set_application_name: crate::constants::APP_NAME,
            set_version: crate::constants::VERSION,

            set_website: "https://github.com/AdwaiGramDevelopers/AdwaiGram/",
            set_issue_url: "https://github.com/AdwaiGramDevelopers/AdwaiGram/issues",

            set_developers: &[
                "Finenko Fedor https://github.com/Sk7Str1p3",
                "Marco Melorio (orig. author) https://github.com/melix99",
                "Marcus Behrendt https://github.com/marhkb",
            ],
            set_artists: &[
                "Mateus Santos https://github.com/swyknox",
                "noëlle https://github.com/jannuary",
            ],
            set_copyright: "© 2025 Finenko Fedor",
            set_license_type: gtk::License::Gpl30,
        }
    }

    fn init(
        parent: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model: AboutPage = AboutPage;
        let widgets: AboutPageWidgets = view_output!();

        root.present(Some(&parent));
        ComponentParts { model, widgets }
    }
}
