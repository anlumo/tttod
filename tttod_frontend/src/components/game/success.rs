use ybc::{HeaderSize, TileCtx};
use yew::prelude::*;

pub struct Success {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub set_ready: Callback<()>,
}

pub enum Msg {
    Ready,
}

impl Component for Success {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ready => {
                self.props.set_ready.emit(());
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let ready_callback = self.link.callback(|_| Msg::Ready);
        html! {
            <ybc::Tile vertical=true ctx=TileCtx::Parent>
                <ybc::Tile vertical=false ctx=TileCtx::Child>
                    <ybc::Title size=HeaderSize::Is1>{"You Have Escaped the Temple of Doom!"}</ybc::Title>
                    <p>
                        {"The Ancient Evil is defeated once and for all! Those alive stagger out of the cursed temple into the bright sunlight, \
                        wondering how this exploit will affect their careers."}
                    </p>
                    <ybc::Button onclick=ready_callback>{"Publish the Groundbreaking Paper"}</ybc::Button>
                </ybc::Tile>
            </ybc::Tile>
        }
    }
}
