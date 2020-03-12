use yew::{html, Component, ComponentLink, Html};

pub struct FragmentsDemo {}

const FRAGMENT_STYLE: &str = "width: 100px; height: 70px; margin: 5px; border: 1px solid red; display: flex; justify-content: center; align-items: center;";
const OTHER_STYLE: &str = "width: 100px; height: 70px; margin: 5px; border: 1px solid blue; display: flex; justify-content: center; align-items: center;";

impl Component for FragmentsDemo {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        FragmentsDemo {}
    }

    fn update(&mut self, _: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div style="display: flex; width: 333px; flex-direction: row; flex-wrap: wrap; border: 1px solid green;">
                     <div class="card" style=OTHER_STYLE>
                        {"We"}
                     </div>
                     {sub_fragments()}
                     <div class="card" style=OTHER_STYLE>
                        {"items."}
                     </div>
                </div>
                <div style="margin: 16px 5px;">{"...but the red ones are in their own fragment!"}</div>
            </div>
        }
    }
}

fn sub_fragments() -> Html {
    html! {
        <>
            <div class="card" style=FRAGMENT_STYLE>
                {"are"}
            </div>
            <div class="card" style=FRAGMENT_STYLE>
                {"all"}
            </div>
            <div class="card" style=FRAGMENT_STYLE>
                {"flex"}
            </div>
        </>
    }
}
