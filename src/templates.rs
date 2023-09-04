use relm4::{gtk, gtk::prelude::*, WidgetTemplate};

#[relm4::widget_template(pub)]
impl WidgetTemplate for MainWindow {
    view! {
        #[root]
        gtk::Window {
            set_title: Some("Application"),
            set_size_request: (1024, 900),
        }
    }
}
