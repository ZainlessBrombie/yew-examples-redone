use yew::agent::{Agent, AgentLink, HandlerId};

pub struct CalculationAgent {
    link: AgentLink<CalculationAgent>,
}

impl Agent for CalculationAgent {
    type Reach = yew::worker::Private;
    type Message = ();
    type Input = (i64, i64);
    type Output = (i64);

    fn create(link: AgentLink<Self>) -> Self {
        CalculationAgent { link }
    }

    fn update(&mut self, _: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        self.link.respond(id, ack(msg.0, msg.1))
    }
}

fn ack(a: i64, b: i64) -> i64 {
    return if a == 0 {
        b + 1
    } else if b == 0 {
        ack(a - 1, 1)
    } else {
        ack(a - 1, ack(a, b - 1))
    };
}
