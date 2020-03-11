use crate::demos::pub_sub::event_bus::EventBus;
use yew::agent::{Bridge, Bridged};
use yew::{html, Component, ComponentLink, Html};

pub struct SubscriberComponent {
    counter: i32,
    _bridge: Box<dyn Bridge<EventBus>>,
}

impl Component for SubscriberComponent {
    type Message = i32;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|a| a);
        let bridge = EventBus::bridge(callback);
        SubscriberComponent {
            counter: 0,
            _bridge: bridge,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        self.counter += msg;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="card" style="height: 300px; width: 300px;">
                <div class="card-content" style="display: flex; align-items: center; flex-direction: column;">
                    <h3>{"Subscriber"}</h3>
                    <h2 style="margin-top: 5px;">{self.counter}</h2>
                </div>
            </div>
        }
    }
}
