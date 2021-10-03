use yew::prelude::*;

#[derive(Debug)]
pub enum AppMsg {
    Increment,
}

pub struct App {
    counter: u64,
    link: ComponentLink<Self>,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { counter: 0, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMsg::Increment => {
                self.counter += 1;
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let increment = self.link.callback(|_| AppMsg::Increment);

        html! {
            <div>
                {format!("Current Counter: {}", self.counter)}
                <br />
                <button onclick=increment>{"Increment"}</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
