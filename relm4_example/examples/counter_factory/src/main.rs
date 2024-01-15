mod counter;
use counter::{Counter, CounterOutput};

use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{gtk, factory::{FactoryVecDeque, DynamicIndex}, RelmApp, ComponentParts, ComponentSender, SimpleComponent};

struct App {
    created_widgets: u8,
    counters: FactoryVecDeque<Counter>,
}

#[derive(Debug)]
pub enum AppMsg {
    AddCounter,
    RemoveCounter,
    SendFront(DynamicIndex),
    MoveUp(DynamicIndex),
    MoveDown(DynamicIndex),
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = u8;
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Factory Example"),
            set_default_size: (300, 100),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,

                gtk::Button {
                    set_label: "Add Counter",
                    connect_clicked => AppMsg::AddCounter,
                },

                gtk::Button {
                    set_label: "Remove Counter",
                    connect_clicked => AppMsg::RemoveCounter,
                },

                #[local_ref]
                counter_box -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                }
            }
        }
    }

    fn init(counter: Self::Init, root: &Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let counters = FactoryVecDeque::builder()
            .launch_default()
            .forward(sender.input_sender(), |msg| match msg {
                CounterOutput::SendFront(index) => AppMsg::SendFront(index),
                CounterOutput::MoveUp(index) => AppMsg::MoveUp(index),
                CounterOutput::MoveDown(index) => AppMsg::MoveDown(index),
            });
        let model = App {
            created_widgets: counter,
            counters,
        };

        let counter_box = model.counters.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        let mut counters_guard = self.counters.guard();
        match msg {
            AppMsg::AddCounter => {
                counters_guard.push_back(self.created_widgets);
                self.created_widgets = self.created_widgets.wrapping_add(1);
            }
            AppMsg::RemoveCounter => {
                counters_guard.pop_back();
            }
            AppMsg::SendFront(index) => {
                counters_guard.move_front(index.current_index());
            }
            AppMsg::MoveDown(index) => {
                let index = index.current_index();
                let new_index = index + 1;
                if new_index < counters_guard.len() {
                    counters_guard.move_to(index, new_index);
                }
            }
            AppMsg::MoveUp(index) => {
                let index = index.current_index();
                if index != 0 {
                    counters_guard.move_to(index, index - 1);
                }
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.factory");
    app.run::<App>(0);
}
