use gtk4 as gtk;
use gtk::prelude::*;
use gtk::glib;
use webkit6 as webkit;
use webkit::prelude::*;

const APP_ID: &str = "xyz.ianhemm.surfer";

fn main() -> glib::ExitCode {
    let url_request = "https://example.com";

    let app = gtk::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(move |app| {
        // all declarative currently. ill move to xml format later(tm)

        // containing box
        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .homogeneous(false)
            .build();

        let headerbar = build_url_header();
        let webview_window = build_webview_window(url_request);

        let window = gtk::Window::builder()
            .application(app)
            .title("surfer")
            .default_width(800)
            .default_height(600)
            .child(&vbox)
            .build();


        vbox.append(&headerbar);
        vbox.append(&webview_window);

        window.present();
    });

    app.run()
}

fn build_url_header() -> gtk::HeaderBar {
    // button and url box
    let toolbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .homogeneous(false)
        .build();
    let back_button = gtk::Button::builder()
        .icon_name("back")
        .build();
    back_button.connect_clicked(|_| {
        dbg!("back button clicked!");
        todo!();
    });

    let forward_button = gtk::Button::builder()
        .icon_name("forward")
        .build();
    forward_button.connect_clicked(|_| {
        dbg!("forward button clicked!");
        todo!();
    });

    let home_button = gtk::Button::builder()
        .icon_name("home")
        .build();
    home_button.connect_clicked(|_| {
        dbg!("home button clicked!");
        todo!();
    });

    let url_text_input = gtk::Entry::builder().build();
    let url_go_button = gtk::Button::builder()
        .icon_name("home")
        .build();
    url_go_button.connect_clicked(|_| {
        dbg!("url go button clicked!");
        todo!();
    });

    toolbox.append(&back_button);
    toolbox.append(&forward_button);
    toolbox.append(&home_button);

    toolbox.append(&url_text_input);
    toolbox.append(&url_go_button);

    // the search bar box
    let search_box = gtk::Box::builder().build();
    let search_text_input = gtk::Entry::builder().build();
    let search_go_button = gtk::Button::builder()
        .icon_name("arrow_left")
        .build();
    search_go_button.connect_clicked(|_| {
        dbg!("search go button clicked!");
        todo!();
    });

    search_box.append(&search_text_input);
    search_box.append(&search_go_button);

    // headerbar setup
    let headerbar = gtk::HeaderBar::builder()
        .show_title_buttons(false)
        .build();
    headerbar.pack_start(&toolbox);
    headerbar.pack_end(&search_box);

    headerbar
}

fn build_webview_window(url_request: &str) -> gtk::ScrolledWindow {
    let webview = webkit::WebView::new();
    webview.load_uri(&url_request);
    let window = gtk::ScrolledWindow::builder()
        .vexpand(true)
        .child(&webview)
        .build();

    window
}
