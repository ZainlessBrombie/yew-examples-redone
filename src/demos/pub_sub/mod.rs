use crate::demos::pub_sub::publisher::PublisherComponent;
use crate::demos::pub_sub::subscriber::SubscriberComponent;
use yew::{html, Component, ComponentLink, Html};

mod event_bus;
mod publisher;
mod subscriber;

pub struct PubSubExample {}

impl Component for PubSubExample {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        return PubSubExample {};
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div style="display: flex; flex-direction: row;">
                <PublisherComponent />
                <div style="width: 100px;"></div>
                <div style="height: 300px; width: 1px; background-color: black; margin-top: .5rem;"></div>
                <div style="width: 100px;"></div>
                <SubscriberComponent />
            </div>
        }
    }
}
