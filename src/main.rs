use std::collections::VecDeque;
use std::mem;

use gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};
use gtk::glib::clone;

use webkit6 as webkit;
use webkit::prelude::*;

const APP_ID: &str = "xyz.ianhemm.surfer";

#[derive(Debug)]
struct AppModel {
    url: String,
    url_textinput: String,
    history: VecDeque<String>,
    future: VecDeque<String>,
    home: String,
}

#[derive(Debug)]
enum Message {
    UrlRequest,
    UrlChange(String),
    Forward,
    Backward,
    Home,
}


#[relm4::component]
impl SimpleComponent for AppModel {
    type Input = Message;
    type Output = ();
    type Init = String;
    type Widgets = AppWidgets;

    view! {
        gtk::Window {
            set_title: Some("surfer"),
            set_default_width: 800,
            set_default_height: 600,
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_homogeneous: false,
                gtk::Box {
                    set_spacing: 5,
                    set_margin_all: 2,
                    set_orientation: gtk::Orientation::Horizontal,

                    gtk::Button {
                        connect_clicked => Message::Backward,
                    },

                    gtk::Button {
                        connect_clicked => Message::Forward,
                    },

                    gtk::Button {

                    },

                    #[name="url_entry"]
                    gtk::Entry {
                        connect_activate => Message::UrlRequest,
                    },

                    gtk::Button {

                    },

                    gtk::Entry {

                    },

                    gtk::Button {

                    },

                },

                gtk::ScrolledWindow {
                    set_vexpand: true,

                    // TODO: connect redirects and loading from web browser
                    // to URL bar and history
                    #[name="web_view"]
                    webkit::WebView {
                        #[watch]
                        load_uri: &format!("{}", model.url),
                    },
                },

            }

        }
    }

    fn init(
        url: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {
            url: url.clone(),
            home: url.clone(), // TODO, allow the user to set a default home
            url_textinput: url,
            future: VecDeque::new(),
            history: VecDeque::new(),
        };


        let widgets = view_output!();
        widgets.url_entry.connect_changed(clone!(@strong sender => move |entry| {
            sender.input(Message::UrlChange(String::from(entry.text().as_str())));
        }));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>){
        match msg {
            Message::UrlRequest => {
                if(!self.url_textinput.is_empty()){
                    self.history.push_back(self.url.clone());
                    self.url = self.url_textinput.clone();
                    self.future.clear();
                }
            },
            Message::UrlChange(url) => {
                self.url_textinput = url;
            },
            Message::Forward => {
                if let Some(mut future_url) = self.future.pop_back() {
                    mem::swap(&mut future_url, &mut self.url);
                    self.history.push_back(future_url);
                }
            },
            Message::Backward => {
                if let Some(mut history_url) = self.history.pop_back() {
                    mem::swap(&mut history_url, &mut self.url);
                    self.future.push_back(history_url);
                }
            },
            Message::Home => {
                self.history.push_back(self.url.clone());
                self.url = self.home.clone();
                self.future.clear();
            },
        };
    }
}


fn main() {
    let app = RelmApp::new(APP_ID);
    app.run::<AppModel>(String::from("https://example.com"));
}
