use gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

use webkit6 as webkit;
use webkit::prelude::*;

const APP_ID: &str = "xyz.ianhemm.surfer";

struct AppModel {
    url: String,
}


#[relm4::component]
impl SimpleComponent for AppModel {
    type Input = ();
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
                    set_orientation: gtk::Orientation::Horizontal,

                    gtk::Button {

                    },

                    gtk::Button {

                    },

                    gtk::Button {

                    },

                    gtk::Entry {

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

                    webkit::WebView {
                        load_uri: "https://example.com/",
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
        let model = AppModel { url };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>){

    }
}


fn main() {
    let app = RelmApp::new(APP_ID);
    app.run::<AppModel>(String::from(""));
}
