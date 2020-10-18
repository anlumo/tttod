mod lobby;
pub use lobby::Lobby;

use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use ws_stream_wasm::WsMeta;
use yew::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    PlayerSelection,
    QuestionsAndAnswers,
    CharacterCreation,
    Game,
}

pub struct Game {
    link: ComponentLink<Self>,
    props: Props,
    state: GameState,
    player_id: Uuid,
    websocket: Option<WsMeta>,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub game_name: String,
}

pub enum Msg {
    SetPlayerName(String),
    SetWebsocket(WsMeta),
}

fn local_storage() -> web_sys::Storage {
    web_sys::window().unwrap().local_storage().unwrap().unwrap()
}

impl Component for Game {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = local_storage();
        let player_id = if let Some(player_id) = storage
            .get_item("player_id")
            .unwrap()
            .and_then(|player_id| Uuid::parse_str(&player_id).ok())
        {
            player_id
        } else {
            let player_id = Uuid::new_v4();
            storage
                .set_item("player_id", &format!("{}", player_id))
                .unwrap();
            player_id
        };
        let inner_link = link.clone();
        let game_name = props.game_name.clone();
        spawn_local(async move {
            let base = {
                let host = "localhost:8081"; //web_sys::window().unwrap().location().host().unwrap();
                if web_sys::window().unwrap().location().protocol().unwrap() == "https:" {
                    format!("wss://{}", host)
                } else {
                    format!("ws://{}", host)
                }
            };
            if let Ok((meta, _)) = WsMeta::connect(
                &format!("{}/api/{}/{}/ws", base, game_name, player_id),
                None,
            )
            .await
            {
                inner_link.send_message(Msg::SetWebsocket(meta));
            }
        });
        Self {
            link,
            props,
            state: GameState::PlayerSelection,
            player_id,
            websocket: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetPlayerName(name) => {
                // TODO
                true
            }
            Msg::SetWebsocket(meta) => {
                self.websocket = Some(meta);
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            // TODO
        }
    }

    fn view(&self) -> Html {
        let set_name_callback = self.link.callback(Msg::SetPlayerName);
        html! {
            <ybc::Tile classes="top-level" vertical=false>
            {
                match self.state {
                    GameState::PlayerSelection => {
                        html! {
                            <Lobby set_name=set_name_callback/>
                        }
                    }
                    GameState::QuestionsAndAnswers => {
                        html! {
                            <div/>
                        }
                    }
                    GameState::CharacterCreation => {
                        html! {
                            <div/>
                        }
                    }
                    GameState::Game => {
                        html! {
                            <div/>
                        }
                    }
                }
            }
            </ybc::Tile>
        }
    }
}
