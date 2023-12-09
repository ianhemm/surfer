use std::env;
use std::error::Error;

use gtk4 as gtk;
use gtk::{prelude::*, ScrolledWindow};
use gtk::{glib, Application, ApplicationWindow};
use webkit6 as webkit;
use webkit::prelude::*;

const APP_ID: &str = "xyz.ianhemm.surfer";

fn main() -> glib::ExitCode {
    let url_request = "https://example.com";

    let app = gtk::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(move |app| {
        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .homogeneous(false)
            .build();

        let headerbar = gtk::HeaderBar::builder()
            .show_title_buttons(false)
            .build();

        let webview = webkit::WebView::new();
        webview.load_uri(&url_request);
        let window = gtk::ScrolledWindow::builder()
            .vexpand(true)
            .child(&webview)
            .build();

        vbox.append(&headerbar);
        vbox.append(&window);

        let window = gtk::Window::builder()
            .application(app)
            .title("surfer")
            .default_width(800)
            .default_height(600)
            .child(&vbox)
            .build();

        window.present();
    });

    app.run()
}
