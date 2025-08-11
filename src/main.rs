use relm4::prelude::*;
use adw::prelude::*;

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

    fn init(
        _init: (),
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = MainWindow;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}

fn main() {
    relm4_icons::initialize_icons();

    let app = RelmApp::new("app.AdwaiGramDevelopers.AdwaiGram");
    app.run::<MainWindow>(());
}
