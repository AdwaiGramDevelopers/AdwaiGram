use chrono;
use relm4::adw;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::gtk::glib::random_int;
use relm4::gtk::prelude::*;
use relm4::typed_view::list::TypedListView;
use relm4::{ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

mod chat_list_item;
use crate::chat_list_item::ChatListItem;

#[derive(Debug)]
struct App {
    chat_list: TypedListView<ChatListItem, gtk::SingleSelection>,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("Simple app"),
            set_default_size: (300, 200),

            adw::NavigationSplitView {
                set_min_sidebar_width: 250f64,
                #[wrap(Some)]
                set_sidebar = &adw::NavigationPage {
                    set_title: "Chats",
                    adw::ToolbarView {
                        add_top_bar = &adw::HeaderBar {
                            #[wrap(Some)]
                            set_title_widget = &gtk::SearchBar {
                                set_search_mode: true,

                                #[wrap(Some)]
                                set_child = &gtk::Box {
                                    add_css_class: "linked",

                                    gtk::SearchEntry {
                                        set_hexpand: true,
                                        set_placeholder_text: Some("Search"),
                                    },
                                }
                            },
                        },

                        gtk::ScrolledWindow {
                            set_vexpand: true,
                            #[local_ref]
                            chats_view -> gtk::ListView {
                                add_css_class: "navigation-sidebar"
                            }
                        }
                    }
                },

                #[wrap(Some)]
                set_content = &adw::NavigationPage {
                    set_title: "ChatName",
                    adw::ToolbarView {
                        add_top_bar = &adw::HeaderBar {
                            #[wrap(Some)]
                            set_title_widget = &adw::WindowTitle {
                                set_title: "Chat"
                            }
                        },
                        add_bottom_bar = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 10,
                            set_margin_all: 10,

                            gtk::TextView {
                                add_css_class: "small-pill",
                                set_wrap_mode: gtk::WrapMode::WordChar,
                                set_valign: gtk::Align::Center,
                                set_vexpand: true,
                                set_hexpand: true,
                            },

                            gtk::Button {
                                add_css_class: "suggested-action",
                                add_css_class: "small-pill",
                            }
                        }

                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = App {
            chat_list: TypedListView::with_sorting(),
        };
        for i in 1..10 {
            model.chat_list.append(ChatListItem {
                chat_name: format!("Chat {i}"),
                last_author: format!("User {i}"),
                last_content: format!("Message {i}"),
                last_time: chrono::Utc::now().timestamp() - 120 * i,
                unread_count: random_int() % 3,
            });
        }
        let chats_view = &model.chat_list.view;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}

fn main() {
    let app = RelmApp::new("app.adwaigramdevs.adwaigram");

    // gtk::gio::resources_register_include!("compiled.gresource")
    //     .expect("Failed to register resources");

    // let provider = gtk::CssProvider::new();
    // provider.load_from_resource("/app/adwaigram/style.css");
    // gtk::style_context_add_provider_for_display(
    //     &gtk::gdk::Display::default().unwrap(),
    //     &provider,
    //     gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    // );

    app.run::<App>(());
}
