use crate::demos::multithreading::multithreading_agent::CalculationAgent;
use crate::yew::Bridged;
use yew::{html, Component, ComponentLink, Html};

mod multithreading_agent;

pub struct MultiThreadingDemo {
    result: Option<i64>,
}

impl Component for MultiThreadingDemo {
    type Message = i64;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut bridge = CalculationAgent::bridge(link.callback(|result| result));
        bridge.send((4, 0));
        MultiThreadingDemo { result: None }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        self.result = Some(msg);
        true
    }

    fn view(&self) -> Html {
        let result_html;
        if let Some(result) = self.result {
            result_html = html! {
                <h3>{result}</h3>
            }
        } else {
            result_html = html! {
                <h3><i>{"Computing..."}</i></h3>
            }
        }
        return html! {
            <div class="card">
                <div class="card-content">
                    <h3>{"Computing ack(4, 0)"}</h3>
                    {result_html}
                </div>
            </div>
        };
    }
}
