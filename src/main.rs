mod ui;
mod constants;

use adw::prelude::*;
use relm4::prelude::*;
use ui::{about::AboutPage, icons};

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
                    let parent = adw::Dialog::default();
                    parent.set_visible(true);
                    let _ = AboutPage::builder().launch(parent);
                }
            }
        }
    }

    fn init(_init: (), root: Self::Root, _sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model: MainWindow = MainWindow;
        let widgets: MainWindowWidgets = view_output!();
        ComponentParts { model, widgets }
    }
}

fn main() {
    let app: RelmApp<()> = RelmApp::new(constants::APP_ID);

    relm4_icons::initialize_icons(icons::GRESOURCE_BYTES, icons::RESOURCE_PREFIX);

    app.run::<MainWindow>(());
}
