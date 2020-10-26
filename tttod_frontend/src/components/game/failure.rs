use ybc::{HeaderSize, TileCtx};
use yew::prelude::*;

pub struct Failure {
    props: Props,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub set_ready: Callback<()>,
}

impl Component for Failure {
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
                    <ybc::Title size=HeaderSize::Is1>{"You Have Failed in the Temple of Doom!"}</ybc::Title>
                    <p class="block">
                        {"The world is consumed by malevolent wrath. The GM(s) explain(s) how this happens."}
                    </p>
                    <div class="failure-image"></div>
                    <ybc::Button onclick=self.props.set_ready.reform(|_| ())>{"Despair"}</ybc::Button>
                </ybc::Tile>
            </ybc::Tile>
        }
    }
}
