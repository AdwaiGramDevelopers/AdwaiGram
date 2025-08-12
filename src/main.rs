use adw::prelude::*;
use relm4::prelude::*;

struct AboutPage;

#[relm4::component]
impl SimpleComponent for AboutPage {
    type Init = adw::Dialog;
    type Input = ();
    type Output = ();

    view! {
        adw::AboutDialog {
            // FIXME: relm4-icons bundles wrong icon, or render it in bad way
            set_application_icon: "logo-devel",
            set_application_name: "AdwaiGram",
            set_developer_name: "AdwaiGram developers",
            set_version: "0.0.1-alpha1",
            set_website: "https://github.com/AdwaiGramDevelopers/AdwaiGram/",
            set_support_url: "https://t.me/paperplanechat",
            set_issue_url: "https://github.com/AdwaiGramDevelopers/AdwaiGram/issues",
            set_developers: &["Finenko Fedor https://github.com/Sk7Str1p3"],
            set_artists: &[
                "Mateus Santos https://github.com/swyknox",
                "noëlle https://github.com/jannuary",
            ],
        }
    }

    fn init(
        parent: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AboutPage;
        let widgets = view_output!();
        // Сразу показываем диалог при инициализации
        root.present(Some(&parent));
        ComponentParts { model, widgets }
    }
}

#[derive(Debug)]
struct MainWindow;

#[relm4::component]
impl SimpleComponent for MainWindow {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("AdwaiGram"),
            set_default_size: (400, 300),

            gtk::Button {
                set_label: "About",
                connect_clicked => move |_| {
                    // Запускаем компонент AboutPage с передачей текущего окна
                    let parent = adw::Dialog::default();
                    parent.set_visible(true); // чтобы было куда крепить
                    let _ = AboutPage::builder().launch(parent);
                }
            }
        }
    }

    fn init(_init: (), root: Self::Root, _sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = MainWindow;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}

mod icons {
    include!(concat!(env!("OUT_DIR"), "/icons.rs"));
}

fn initialize_custom_icons() {
    gtk::gio::resources_register_include!("icons.gresource").unwrap();

    let display = gtk::gdk::Display::default().unwrap();
    let theme = gtk::IconTheme::for_display(&display);
    theme.add_resource_path("/app/AdwaiGramDevelopers/AdwaiGram/icons");
}

fn main() {
    let app = RelmApp::new("app.AdwaiGramDevelopers.AdwaiGram");

    //relm4_icons::initialize_icons(icons::GRESOURCE_BYTES, icons::RESOURCE_PREFIX);
    initialize_custom_icons();

    app.run::<MainWindow>(());
}
