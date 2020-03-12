use serde::{Deserialize, Serialize};
use yew::format::Json;
use yew::services::storage::Area;
use yew::services::StorageService;
use yew::{html, Component, ComponentLink, Html, InputData};

const KEY: &'static str = "yew.crm.database";

#[derive(Serialize, Deserialize)]
struct Database {
    clients: Vec<Client>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Client {
    first_name: String,
    last_name: String,
    order: String,
}

impl Client {
    fn empty() -> Client {
        Client {
            first_name: "".to_string(),
            last_name: "".to_string(),
            order: "".to_string(),
        }
    }
}

pub enum CrmAction {
    UpdateFirstName(String),
    UpdateLastName(String),
    UpdateOrder(String),
    PlaceOrder,
    RemoveOrder(usize),
}

pub struct CrmExample {
    database: Database,
    storage: StorageService,
    link: ComponentLink<CrmExample>,
    current_first_name: String,
    current_last_name: String,
    current_order: String,
}

impl CrmExample {
    fn can_add(&self) -> bool {
        return self.current_order.len() > 0
            && self.current_first_name.len() > 0
            && self.current_last_name.len() > 0;
    }

    fn submit(&mut self) {
        self.database.clients.push(Client {
            first_name: self.current_first_name.clone(),
            last_name: self.current_last_name.clone(),
            order: self.current_order.clone(),
        });
        self.storage.store(KEY, Json(&self.database));

        self.current_order = "".to_string();
        self.current_first_name = "".to_string();
        self.current_last_name = "".to_string();
    }
}

impl Component for CrmExample {
    type Message = CrmAction;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let Json(database) = storage.restore(KEY);
        let database = database.unwrap_or_else(|_| Database {
            clients: vec![Client {
                first_name: "Joline".to_string(),
                last_name: "Doe".to_string(),
                order: "Three pounds of bits".to_string(),
            }],
        });

        CrmExample {
            database,
            storage,
            link,
            current_first_name: "".to_string(),
            current_last_name: "".to_string(),
            current_order: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            CrmAction::UpdateFirstName(first_name) => self.current_first_name = first_name,
            CrmAction::UpdateLastName(last_name) => self.current_last_name = last_name,
            CrmAction::UpdateOrder(order_text) => self.current_order = order_text,
            CrmAction::PlaceOrder => {
                if self.can_add() {
                    self.submit();
                }
            }
            CrmAction::RemoveOrder(index) => {
                self.database.clients.remove(index);
            }
        }
        true
    }

    fn view(&self) -> Html {
        let mut clients_list: Vec<Html> = Vec::new();

        for (id, client) in self.database.clients.iter().enumerate() {
            clients_list.push(html! {
                <div style="margin-top: 16px; width: 34vw;"
                     class="card">
                    <div class="card-content">
                        {format!("{} {}", client.first_name, client.last_name)}
                        <p>{client.order.clone()}</p>
                        <button style="margin-top: 16px;"
                                class="waves-effect waves-light btn red"
                                onclick=self.link.callback(move |evt: web_sys::MouseEvent| CrmAction::RemoveOrder(id))>
                            {"Cancel Order"}
                        </button>
                    </div>
                </div>
            });
        }

        return html! {
            <div style="height: 70vh; width: 70vw;">
                <h3>{"Customer Orders"}</h3>
                <div class="card">
                    <div class="card-content">
                        <div class="input-field">
                            <input id="first_name" type="text"
                                   oninput=self.link.callback(|evt: InputData| {CrmAction::UpdateFirstName(evt.value)})
                                   value=self.current_first_name.clone()
                                   placeholder="First Name"/>
                            <label for="first_name"></label>
                        </div>
                        <div class="input-field">
                            <input id="last_name" type="text"
                                   oninput=self.link.callback(|evt: InputData| {CrmAction::UpdateLastName(evt.value)})
                                   value=self.current_last_name.clone()
                                   placeholder="Last Name"/>
                            <label for="last_name"></label>
                        </div>
                        <div>
                            <textarea id="textarea1" class="materialize-textarea"
                                      oninput=self.link.callback(|evt: InputData| {CrmAction::UpdateOrder(evt.value)})
                                      value=self.current_order.clone()/>
                            <label for="textarea1">{"Order"}</label>
                        </div>
                        <button style="margin-top: 16px;"
                                class="waves-effect waves-light btn"
                                onclick=self.link.callback(|evt: web_sys::MouseEvent| CrmAction::PlaceOrder)>
                            {"Submit"}
                        </button>
                    </div>
                </div>
                <div style="display: flex; flex-direction: row; flex-wrap: wrap; justify-content: space-between;">
                    {for clients_list}
                </div>
            </div>
        };
    }
}
