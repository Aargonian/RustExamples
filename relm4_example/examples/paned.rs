use gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, SimpleComponent};

struct PanedModel {}

#[derive(Debug)]
enum PanedMessage {}

#[relm4::component]
impl SimpleComponent for PanedModel {
    type Init = ();
    type Input = PanedMessage;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Paned Example"),
            set_default_width: 300,
            set_default_height: 200,

            gtk::Paned {
                set_orientation: gtk::Orientation::Horizontal,
                set_resize_start_child: false,
                set_shrink_start_child: false,
                set_resize_end_child: true,

                #[wrap(Some)]
                set_start_child = &gtk::Label {
                    set_label: "Left Side",
                    set_size_request: (100, -1)
                },

                #[wrap(Some)]
                set_end_child = &gtk::Label {
                    set_label: "Right Side"
                }
            }
        }
    }

    fn init(_: Self::Init, root: &Self::Root, _sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = PanedModel {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>) {
        // No update logic needed for this static UI
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.paned");
    app.run::<PanedModel>(());
}
