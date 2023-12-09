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




        // the URL and search bar
        let toolbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .homogeneous(false)
            .build();
        let back_button = gtk::Button::builder()
            .icon_name("back")
            .build();
        let forward_button = gtk::Button::builder()
            .icon_name("forward")
            .build();
        let home_button = gtk::Button::builder()
            .icon_name("home")
            .build();

        let url_text_input = gtk::Entry::builder().build();
        let search_text_input = gtk::Entry::builder().build();

        toolbox.append(&back_button);
        toolbox.append(&forward_button);
        toolbox.append(&home_button);

        toolbox.append(&url_text_input);

        let headerbar = gtk::HeaderBar::builder()
            .show_title_buttons(false)
            .build();
        headerbar.pack_start(&toolbox);
        headerbar.pack_end(&search_text_input);

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
