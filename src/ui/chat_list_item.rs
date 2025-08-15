use relm4::adw;
use relm4::gtk::prelude::*;
use relm4::{self, RelmWidgetExt};
use relm4::{gtk, typed_view::list::RelmListItem};

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct ChatListItem {
    pub chat_name: String,
    pub last_author: String,
    pub last_content: String,
    pub last_time: i64,
    pub unread_count: u32,
}

pub struct ChatListItemWidgets {
    chat_avatar: adw::Avatar,
    chat_name_label: gtk::Label,
    last_author_label: gtk::Label,
    last_content_label: gtk::Label,
    unread_count_label: gtk::Label,
}

impl RelmListItem for ChatListItem {
    type Root = gtk::Box;
    type Widgets = ChatListItemWidgets;

    fn setup(_: &gtk::ListItem) -> (Self::Root, Self::Widgets) {
        relm4::view! {
            hbox = gtk::Box {
                set_margin_all: 5,
                set_spacing: 10,
                set_orientation: gtk::Orientation::Horizontal,

                #[name = "chat_avatar"]
                adw::Avatar {
                    set_size: 48,
                    set_show_initials: true,
                    //set_custom_image: Some(&gdk::Texture::from_filename("images.png")
                    //                .expect("Failed to load image")),
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_valign: gtk::Align::Center,
                    set_hexpand: true,

                    #[name = "chat_name_label"]
                    gtk::Label {
                        add_css_class: "heading",
                        set_halign: gtk::Align::Start,
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        #[name = "last_author_label"]
                        gtk::Label,
                        #[name = "last_content_label"]
                        gtk::Label {
                            set_ellipsize: gtk::pango::EllipsizeMode::End,
                            add_css_class: "dim-label",
                        },
                    }
                },

                #[name = "unread_count_label"]
                gtk::Label {
                    set_valign: gtk::Align::Center,
                    add_css_class: "badge",
                }
            }
        }

        let widgets = ChatListItemWidgets {
            chat_avatar,
            chat_name_label,
            last_author_label,
            last_content_label,
            unread_count_label,
        };

        (hbox, widgets)
    }

    fn bind(&mut self, _widgets: &mut Self::Widgets, _root: &mut Self::Root) {
        _widgets.chat_avatar.set_text(Some(&self.chat_name));
        _widgets.chat_name_label.set_text(&self.chat_name);
        _widgets.last_content_label.set_text(&self.last_content);
        _widgets
            .last_author_label
            .set_text(&format!("{}: ", &self.last_author));
        if self.unread_count != 0 {
            _widgets
                .unread_count_label
                .set_text(&self.unread_count.to_string());
        }
        _widgets
            .unread_count_label
            .set_visible(self.unread_count > 0);
    }
}

impl Ord for ChatListItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.last_time.cmp(&other.last_time)
    }
}
