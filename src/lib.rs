#![recursion_limit = "512"]
use yew::{html, Component, ComponentLink, Html, ShouldRender};
extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate yew;
use crate::demos::crm::CrmExample;
use crate::demos::fragments::FragmentsDemo;
use crate::demos::futures::FutureDemo;
use crate::demos::pub_sub::PubSubExample;
use wasm_bindgen::prelude::*;

mod demos;

struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div style="display: flex; height: 100vh; width: 100vw; justify-content: center; align-items: center;">
                <FutureDemo />
            </div>
        }
    }
}

#[wasm_bindgen]
pub fn main() {
    yew::start_app::<App>();
}
