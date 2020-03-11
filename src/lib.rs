#![recursion_limit = "256"]
use yew::{Component, ComponentLink, Html, ShouldRender, html};
extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate yew;
use wasm_bindgen::prelude::*;
use crate::demos::pub_sub::PubSubExample;

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
                <PubSubExample />
            </div>
        }
    }
}

#[wasm_bindgen]
pub fn main() {
    yew::start_app::<App>();
}
