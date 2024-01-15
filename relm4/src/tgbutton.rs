use relm4::gtk;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::Button;

mod imp {
    use super::*;
    
    #[derive(Default)]
    pub struct CustomButton;

    #[glib::object_subclass]
    impl ObjectSubclass for CustomButton {
        const NAME: &'static str = "CustomButton";
        type Type = super::CustomButton;
        type ParentType = Button;
    }

    impl ObjectImpl for CustomButton {}
    impl WidgetImpl for CustomButton {
        fn snapshot(&self, widget: &Self::Type, snapshot: &gtk::Snapshot) {
            self.parent_snapshot(widget, snapshot);

            let rect = widget.allocation();
            let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, rect.width(), rect.height())
                .expect("Can't create surface");
            let cr = cairo::Context::new(&surface).expect("Can't create context");

            // Draw the hollow circle
            cr.set_source_rgb(1.0, 0.0, 0.0); // Red color
            cr.set_line_width(5.0); // Line width for the circle
            cr.arc(
                (rect.width() / 2) as f64,
                (rect.height() / 2) as f64,
                22.5, // Radius
                0.0,
                2.0 * std::f64::consts::PI,
            );
            cr.stroke();

            // Draw on the widget
            let snapshot_rect = gtk::gdk::Rectangle::new(rect.x(), rect.y(), rect.width(), rect.height());
            snapshot.append_cairo(&snapshot_rect, &surface);
        }
    }

    impl ButtonImpl for CustomButton {}
}

glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends Button, gtk::Widget;
}

impl CustomButton {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create CustomButton")
    }
}
pub use imp::CustomButton;
