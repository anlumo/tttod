use crate::{
    components::{Icon, Introduction},
    IconName,
};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::HtmlElement;
use ybc::TileSize;
use yew::prelude::*;

pub struct Player {
    link: ComponentLink<Self>,
    props: Props,
    input_ref: NodeRef,
    player_name: String,
    keyup_closure: Closure<dyn FnMut(web_sys::KeyboardEvent)>,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub set_name: Callback<String>,
}

pub enum Msg {
    UpdateName(String),
    EnterGame,
}

impl Component for Player {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let inner_link = link.clone();
        let keyup_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                inner_link.send_message(Msg::EnterGame);
                event.stop_propagation();
            }
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
        Self {
            link,
            props,
            input_ref: NodeRef::default(),
            player_name: "".to_owned(),
            keyup_closure,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateName(name) => {
                self.player_name = name;
                true
            }
            Msg::EnterGame => {
                self.props.set_name.emit(self.player_name.clone());
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(node) = self.input_ref.get() {
                if let Some(element) = node.dyn_ref::<HtmlElement>() {
                    element.focus().ok();
                    element.set_onkeyup(self.keyup_closure.as_ref().dyn_ref());
                }
            }
        }
    }

    fn view(&self) -> Html {
        let game_callback = self.link.callback(|_| Msg::EnterGame);
        let update_name_callback = self.link.callback(Msg::UpdateName);
        html! {
            <ybc::Tile classes="top-level" vertical=false>
                <Introduction/>
                <ybc::Tile vertical=true size=TileSize::Four>
                    <ybc::Section>
                        <ybc::Field classes="control has-icons-left">
                           <ybc::Input name="game" update=update_name_callback value=self.player_name.clone() placeholder="Player name" rounded=false ref=self.input_ref.clone()/>
                           <span class="icon is-small is-left">
                                <Icon name=IconName::User/>
                            </span>
                       </ybc::Field>
                        <ybc::Field>
                            <ybc::Button disabled=self.player_name.is_empty() onclick=game_callback>{"Enter the Temple"}</ybc::Button>
                        </ybc::Field>
                    </ybc::Section>
                </ybc::Tile>
            </ybc::Tile>
        }
    }
}
