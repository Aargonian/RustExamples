use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

struct CounterModel {
    counter: u8,
}

#[derive(Debug)]
enum CounterMessage {
    Increment,
    Decrement,
}

#[relm4::component]
impl SimpleComponent for CounterModel {
    type Init = u8;

    type Input = CounterMessage;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Counter"),
            set_default_width: 300,
            set_default_height: 100,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button {
                    set_label: "Increment",
                    connect_clicked[sender] => move |_| {
                        sender.input(CounterMessage::Increment);
                    }
                },

                gtk::Button::with_label("Decrement") {
                    connect_clicked[sender] => move |_| {
                        sender.input(CounterMessage::Decrement);
                    }
                },

                gtk::Label {
                    #[watch]
                    set_label: &format!("Counter: {}", model.counter),
                    set_margin_all: 5,
                },

                if model.counter % 2 == 0 {
                    gtk::Label {
                        set_label: "The value is even",
                    }
                } else {
                    gtk::Label {
                        set_label: "The value is odd",
                    }
                }
            }
        }
    }

    fn init(counter: Self::Init, root: &Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = CounterModel { counter };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            CounterMessage::Increment => self.counter = self.counter.wrapping_add(1),
            CounterMessage::Decrement => self.counter = self.counter.wrapping_sub(1),
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple");
    app.run::<CounterModel>(0);
}
