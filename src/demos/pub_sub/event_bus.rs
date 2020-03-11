use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::Debug;
use yew::worker::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    EventBusMsg(i32),
}

pub struct EventBus {
    link: AgentLink<EventBus>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for EventBus {
    type Reach = Context;
    type Message = ();
    type Input = Request;
    type Output = i32;

    fn create(link: AgentLink<Self>) -> Self {
        EventBus {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _: Self::Message) {}

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        match msg {
            Request::EventBusMsg(v) => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, v);
                }
            }
        }
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
