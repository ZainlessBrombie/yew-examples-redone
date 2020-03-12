use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use yew::{html, Component, ComponentLink, Html};

pub struct FutureDemo {
    display_string: String,
}

impl Component for FutureDemo {
    type Message = String;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let future = async move {
            let promise = js_later(js_sys::Promise::resolve(&JsValue::from_str("Hello")));

            let resolved_to: Result<JsValue, JsValue> =
                wasm_bindgen_futures::JsFuture::from(promise).await;
            if let Ok(result) = resolved_to {
                link.send_message(
                    result
                        .as_string()
                        .unwrap_or_else(|| "Err: Result was not a string".to_string()),
                );
            } else {
                link.send_message("Error occurred".to_string());
            }
            Ok(JsValue::undefined())
        };
        wasm_bindgen_futures::future_to_promise(future);
        FutureDemo {
            display_string: "Waiting for promise...".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        self.display_string = msg;
        true
    }

    fn view(&self) -> Html {
        return html! {
            <div>
                <div><b>{self.display_string.clone()}</b></div>
                <div>
                    {"Under the hood, a js function is called with a promise:"}<br/><br/>
                    <code>{"export async function js_later(inp) {"}<br/>
                    {"\u{00a0}\u{00a0}\u{00a0}await new Promise(cb => setTimeout(cb, 2000));"}<br/>
                    {"\u{00a0}\u{00a0}\u{00a0}return (await inp) + ' World';"}<br/>
                    {"}"}</code><br/>
                </div>
            </div>
        };
    }
}

#[wasm_bindgen(inline_js = "export async function js_later(inp) {\
        await new Promise(cb => setTimeout(cb, 2000));\
        return (await inp) + ' World';\
    }")]
extern "C" {
    fn js_later(p: js_sys::Promise) -> js_sys::Promise;
}
