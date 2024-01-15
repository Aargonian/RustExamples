use relm4::gtk::{Application, ApplicationWindow, prelude::{ApplicationExt, GtkWindowExt, ButtonExt, WidgetExt, ApplicationExtManual}};
use tgbutton::CustomButton;

mod tgbutton;

fn main() {
    let application = Application::new(Some("com.example.custombutton"), Default::default());

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title(Some("Custom Button Example"));
        window.set_default_size(300, 100);

        let custom_button = CustomButton::new();
        custom_button.set_label("Custom Button");

        window.set_child(Some(&custom_button));
        window.show();
    });

    application.run();
}
