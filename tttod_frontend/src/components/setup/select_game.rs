use crate::{
    components::{root::AppRoute, Icon, Introduction},
    IconName,
};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::HtmlElement;
use ybc::{TileCtx, TileSize};
use yew::prelude::*;
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::{Route, RouteAgentDispatcher},
};

pub struct SelectGame {
    link: ComponentLink<Self>,
    input_ref: NodeRef,
    game_name: String,
    router: RouteAgentDispatcher,
    keyup_closure: Closure<dyn FnMut(web_sys::KeyboardEvent)>,
}

pub enum Msg {
    UpdateName(String),
    EnterGame,
}

impl Component for SelectGame {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let inner_link = link.clone();
        let keyup_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            log::debug!("key = {}", event.key());
            if event.key() == "Enter" {
                inner_link.send_message(Msg::EnterGame);
                event.stop_propagation();
            }
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
        Self {
            link,
            input_ref: NodeRef::default(),
            game_name: "".to_owned(),
            router: RouteAgentDispatcher::new(),
            keyup_closure,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateName(name) => {
                self.game_name = name;
                true
            }
            Msg::EnterGame => {
                self.router.send(ChangeRoute(Route::from(AppRoute::Game(
                    self.game_name.clone(),
                ))));
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
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
            <ybc::Tile vertical=false ctx=TileCtx::Ancestor>
                <ybc::Tile vertical=false ctx=TileCtx::Parent size=TileSize::Eight>
                    <Introduction/>
                </ybc::Tile>
                <ybc::Tile vertical=false ctx=TileCtx::Parent>
                    <ybc::Tile vertical=true ctx=TileCtx::Child>
                        <ybc::Section>
                            <ybc::Field classes="control has-icons-left">
                            <ybc::Input name="game" update=update_name_callback value=self.game_name.clone() placeholder="Game name" rounded=false ref=self.input_ref.clone()/>
                            <span class="icon is-small is-left">
                                <Icon name=IconName::Gopuram/>
                                </span>
                            </ybc::Field>
                            <ybc::Field>
                                <ybc::Button disabled=self.game_name.is_empty() onclick=game_callback><Icon classes="icon" name=IconName::Hiking/><span>{"Prepare the Expedition"}</span></ybc::Button>
                            </ybc::Field>
                        </ybc::Section>
                    </ybc::Tile>
                </ybc::Tile>
            </ybc::Tile>
        }
    }
}
