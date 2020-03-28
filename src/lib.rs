#![recursion_limit = "512"]
extern crate yew;
use wasm_bindgen::prelude::*;
use web_sys::console;
mod demos;
use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties, Children, Callback};
extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;
use web_sys::{ScrollToOptions};
use crate::demos::multithreading::MultiThreadingDemo;
use wasm_bindgen::prelude::*;
use crate::yew::Renderable;
use crate::demos::crm::CrmExample;
use crate::demos::fragments::FragmentsDemo;
use crate::demos::futures::FutureDemo;
use crate::demos::pub_sub::PubSubExample;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct App {
    link: ComponentLink<App>
}
struct ExamplePage {
    props: ExamplePageProps
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { link }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        let window = web_sys::window().unwrap();
        let height_js: JsValue = window.inner_height().unwrap();
        let height: f64 = height_js.as_f64().unwrap();
        let scroll_to_page = move |page: f64| {
            let mut options = ScrollToOptions::new();
            options.behavior(web_sys::ScrollBehavior::Smooth)
                .top(height * page);
            web_sys::window().unwrap().scroll_with_scroll_to_options(&options);
        };
        let navigation = html! {
            <div style="position: fixed; top: 30px; left: 30px;">
                <h4>{"Examples"}</h4>
                <div class="blue-text lighten-1" style="cursor: pointer; margin-top: 10px;" onclick=self.link.callback(move |_| {
                    scroll_to_page(0.);
                    ()
                })>{"Pub Sub"}</div>
                <div class="blue-text lighten-1" style="cursor: pointer; margin-top: 10px;" onclick=self.link.callback(move |_| {
                    scroll_to_page(1.);
                    ()
                })>{"Crm"}</div>
                <div class="blue-text lighten-1" style="cursor: pointer; margin-top: 10px;" onclick=self.link.callback(move |_| {
                    scroll_to_page(2.);
                    ()
                })>{"Fragments"}</div>
                <div class="blue-text lighten-1" style="cursor: pointer; margin-top: 10px;" onclick=self.link.callback(move |_| {
                    scroll_to_page(3.);
                    ()
                })>{"Futures"}</div>
            </div>
        };

        html! {
            <div>
                {navigation}
                <ExamplePage>
                    <PubSubExample />
                </ExamplePage>
                <ExamplePage>
                    <CrmExample />
                </ExamplePage>
                <ExamplePage>
                    <FragmentsDemo />
                </ExamplePage>
                <ExamplePage>
                    <FutureDemo />
                </ExamplePage>
            </div>
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
struct ExamplePageProps {
    children: Children
}

impl Component for ExamplePage {
    type Message = ();
    type Properties = ExamplePageProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ExamplePage {
            props
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div style="display: flex; height: 100vh; width: 98vw; justify-content: center; align-items: center; border-bottom: 1px solid black;">
                {self.props.children.render()}
            </div>
        }
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // Your code goes here!
    yew::start_app::<App>();

    Ok(())
}
