use std::env;
use std::error::Error;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use webkit6 as webkit;
use webkit::prelude::*;

const APP_ID: &str = "xyz.ianhemm.surfer";

fn main() -> Result<(),Box<dyn Error>>{

    let app = gtk::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(move |app| {
        let webview = webkit::WebView::new();
        webview.load_uri("https://example.com/");

        let window = gtk::Window::builder()
            .application(app)
            .title("surfer")
            .default_width(800)
            .default_height(600)
            .child(&webview)
            .build();

        window.present();
    });

    app.run();
    Ok(())
}

fn parse_args(args: &[String]) -> &str {

    let args: Vec<String> = env::args().collect();
    let uri = parse_args(&args);
    let uri = &args[1];

    uri
}
