use ybc::{HeaderSize, TileCtx};
use yew::prelude::*;

pub struct Success {
    props: Props,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub set_ready: Callback<()>,
}

impl Component for Success {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <ybc::Tile vertical=true ctx=TileCtx::Parent>
                <ybc::Tile vertical=false ctx=TileCtx::Child>
                    <ybc::Title size=HeaderSize::Is1>{"You Have Escaped the Temple of Doom!"}</ybc::Title>
                    <p class="block">
                        {"The Ancient Evil is defeated once and for all! Those alive stagger out of the cursed temple into the bright sunlight, \
                        wondering how this exploit will affect their careers."}
                    </p>
                    <div class="success-image"></div>
                    <ybc::Button onclick=self.props.set_ready.reform(|_| ())>{"Publish the Groundbreaking Paper"}</ybc::Button>
                </ybc::Tile>
            </ybc::Tile>
        }
    }
}
