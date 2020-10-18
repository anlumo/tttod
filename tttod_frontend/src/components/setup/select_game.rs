use crate::components::{root::AppRoute, Introduction};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use ybc::TileSize;
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
}

pub enum Msg {
    UpdateName(String),
    EnterGame,
}

impl Component for SelectGame {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            input_ref: NodeRef::default(),
            game_name: "".to_owned(),
            router: RouteAgentDispatcher::new(),
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
                        <ybc::Field>
                           <ybc::Input name="game" update=update_name_callback value=self.game_name.clone() placeholder="Game name" rounded=false ref=self.input_ref.clone()/>
                        </ybc::Field>
                        <ybc::Field>
                            <ybc::Button disabled=self.game_name.is_empty() onclick=game_callback>{"Prepare the Expedition"}</ybc::Button>
                        </ybc::Field>
                    </ybc::Section>
                </ybc::Tile>
            </ybc::Tile>
        }
    }
}
