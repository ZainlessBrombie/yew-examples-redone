use crate::demos::pub_sub::event_bus::EventBus;
use crate::demos::pub_sub::event_bus::Request::EventBusMsg;
use yew::agent::Dispatched;
use yew::agent::Dispatcher;
use yew::{html, Callback, Component, ComponentLink, Html};

pub struct PublisherComponent {
    bus_dispatcher: Dispatcher<EventBus>,
    call_self_1: Callback<web_sys::MouseEvent>,
    call_self_10: Callback<web_sys::MouseEvent>,
}

impl Component for PublisherComponent {
    type Message = i32;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let bus_dispatcher = EventBus::dispatcher();
        PublisherComponent {
            bus_dispatcher,
            call_self_1: link.callback(|_| 1),
            call_self_10: link.callback(|_| 10),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        self.bus_dispatcher.send(EventBusMsg(msg));
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="card" style="height: 300px; width: 300px;">
                <div class="card-content" style="display: flex; align-items: center; flex-direction: column;">
                    <h3>{"Publisher"}</h3>
                    <button class="waves-effect waves-light btn"
                            style="margin-top: 16px; width: 140px;"
                            onclick=self.call_self_1.clone()>{"Click Me (1)"}</button>
                    <button class="waves-effect waves-light btn mt-2"
                            style="margin-top: 16px; width: 140px;"
                            onclick=self.call_self_10.clone()>{"Click Me (10)"}</button>
                </div>
            </div>
        }
    }
}
