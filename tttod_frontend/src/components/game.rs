mod lobby;
pub use lobby::Lobby;
mod define_evil;
pub use define_evil::DefineEvil;
mod create_character;
pub use create_character::CreateCharacter;
mod player_list;
pub use player_list::PlayerList;
mod introduce_characters;
pub use introduce_characters::IntroduceCharacters;

use futures::{
    sink::SinkExt,
    stream::{SplitSink, StreamExt},
};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};
use tttod_data::{ClientToServerMessage, GameState, Player, PlayerStats, ServerToClientMessage};
use uuid::Uuid;
use wasm_bindgen::{closure::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use ws_stream_wasm::{WsMessage, WsMeta, WsStream};
use ybc::{HeaderSize, TileCtx};
use yew::prelude::*;

pub struct Game {
    link: ComponentLink<Self>,
    props: Props,
    state: GameState,
    player_id: Uuid,
    websocket: Option<(WsMeta, Rc<RefCell<SplitSink<WsStream, WsMessage>>>)>,
    players: HashMap<Uuid, Player>,
    player_kick_votes: HashMap<Uuid, HashSet<Uuid>>,
    questions: Vec<(String, String)>,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub game_name: String,
}

pub enum Msg {
    SetPlayerName(String),
    VoteKick(Uuid),
    PlayerReady,
    SetAnswer(usize, String),
    SetCharacter(PlayerStats),
    SetWebsocket(WsMeta, SplitSink<WsStream, WsMessage>),
    WebsocketClosed,
    ConnectWebsocket,
    ReceivedMessage(ServerToClientMessage),
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
        let instance = Self {
            link,
            props,
            state: GameState::PlayerSelection,
            player_id,
            websocket: None,
            players: HashMap::new(),
            player_kick_votes: HashMap::new(),
            questions: Vec::new(),
        };
        instance.connect_websocket();
        instance
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetPlayerName(name) => {
                self.send_message(ClientToServerMessage::SetPlayerName { name });
                false
            }
            Msg::PlayerReady => {
                self.send_message(ClientToServerMessage::ReadyForGame);
                false
            }
            Msg::VoteKick(player_id) => {
                self.send_message(ClientToServerMessage::VoteKickPlayer { player_id });
                false
            }
            Msg::SetAnswer(idx, text) => {
                let updated = if let Some((_, answer)) = self.questions.get_mut(idx) {
                    *answer = text;
                    true
                } else {
                    false
                };
                if updated {
                    self.send_message(ClientToServerMessage::Answers {
                        answers: self.questions.iter().map(|(_, a)| a.clone()).collect(),
                    });
                }
                updated
            }
            Msg::SetCharacter(stats) => {
                self.send_message(ClientToServerMessage::SetCharacter { stats });
                false
            }
            Msg::SetWebsocket(meta, sink) => {
                self.websocket = Some((meta, Rc::new(RefCell::new(sink))));
                true
            }
            Msg::ReceivedMessage(message) => {
                log::debug!("received message {:?}", message);
                match message {
                    ServerToClientMessage::GameIsFull => false,
                    ServerToClientMessage::GameIsOngoing => false,
                    ServerToClientMessage::EndGame => false,
                    ServerToClientMessage::PushState {
                        players,
                        game_state,
                        player_kick_votes,
                    } => {
                        self.state = game_state;
                        self.players = players;
                        self.player_kick_votes = player_kick_votes;
                        true
                    }
                    ServerToClientMessage::DeclareGM { player_id: gm } => false,
                    ServerToClientMessage::Questions { questions } => {
                        self.questions = questions
                            .into_iter()
                            .map(|(q, a)| (q, a.unwrap_or_default()))
                            .collect();
                        true
                    }
                }
            }
            Msg::WebsocketClosed => {
                let link = self.link.clone();
                let closure = Closure::once_into_js(move || {
                    link.send_message(Msg::ConnectWebsocket);
                });
                web_sys::window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        1000,
                    )
                    .unwrap();
                false
            }
            Msg::ConnectWebsocket => {
                self.connect_websocket();
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn destroy(&mut self) {
        if let Some((meta, _)) = self.websocket.take() {
            spawn_local(async move {
                if let Err(err) = meta.close().await {
                    log::error!("Failed disconnecting websocket: {:?}", err);
                }
            });
        }
    }

    fn view(&self) -> Html {
        let set_name_callback = self.link.callback(Msg::SetPlayerName);
        let set_ready_callback = self.link.callback(|_| Msg::PlayerReady);
        let vote_kick_callback = self.link.callback(Msg::VoteKick);
        let set_answer_callback = self.link.callback(|(idx, text)| Msg::SetAnswer(idx, text));
        let set_character_callback = self.link.callback(Msg::SetCharacter);
        html! {
            <ybc::Tile vertical=false ctx=TileCtx::Ancestor>
            {
                if self.websocket.is_some() {
                    log::debug!("state = {:?}", self.state);
                    match self.state {
                        GameState::PlayerSelection => {
                            html! {
                                <Lobby set_name=set_name_callback set_ready=set_ready_callback vote_kick=vote_kick_callback player_id=self.player_id players=self.players.clone() player_kick_votes=self.player_kick_votes.clone()/>
                            }
                        }
                        GameState::DefineEvil => {
                            html! {
                                <DefineEvil player_id=self.player_id players=self.players.clone() questions=self.questions.clone() set_answer=set_answer_callback set_ready=set_ready_callback/>
                            }
                        }
                        GameState::CharacterCreation => {
                            let stats = if let Some(player) = self.players.get(&self.player_id) {
                                player.stats.clone().unwrap_or_default()
                            } else {
                                PlayerStats::default()
                            };
                            html! {
                                <CreateCharacter stats=stats player_id=self.player_id players=self.players.clone() set_character=set_character_callback set_ready=set_ready_callback/>
                            }
                        }
                        GameState::CharacterIntroduction => {
                            html! {
                                <IntroduceCharacters player_id=self.player_id players=self.players.clone() set_ready=set_ready_callback/>
                            }
                        }
                        GameState::Room(_idx) => {
                            html! {
                                <div/>
                            }
                        }
                        GameState::FinalBattle => {
                            html! {
                                <div/>
                            }
                        }
                    }
                } else {
                    html! {
                        <ybc::Title size=HeaderSize::Is4>{"Connecting to server…"}</ybc::Title>
                    }
                }
            }
            </ybc::Tile>
        }
    }
}

impl Game {
    fn connect_websocket(&self) {
        let game_name = self.props.game_name.clone();
        let link = self.link.clone();
        let player_id = self.player_id;
        spawn_local(async move {
            let base = {
                let host = "localhost:8081"; //web_sys::window().unwrap().location().host().unwrap();
                if web_sys::window().unwrap().location().protocol().unwrap() == "https:" {
                    format!("wss://{}", host)
                } else {
                    format!("ws://{}", host)
                }
            };
            if let Ok((meta, stream)) = WsMeta::connect(
                &format!("{}/api/{}/{}/ws", base, game_name, player_id),
                None,
            )
            .await
            {
                let (sink, mut stream) = stream.split();
                link.send_message(Msg::SetWebsocket(meta, sink));
                while let Some(message) = stream.next().await {
                    if let WsMessage::Text(text) = message {
                        match serde_json::from_str(&text) {
                            Err(err) => {
                                log::error!("Failed parsing json message: {:?}", err);
                            }
                            Ok(message) => {
                                link.send_message(Msg::ReceivedMessage(message));
                            }
                        }
                    } else {
                        log::error!("Unkonwn binary message received");
                    }
                }
                log::warn!("Websocket connection lost!");
                link.send_message(Msg::WebsocketClosed);
            }
        });
    }
    fn send_message(&self, message: ClientToServerMessage) {
        if let Some((_, sender)) = &self.websocket {
            let sender = Rc::downgrade(sender);
            spawn_local(async move {
                if let Some(sender) = sender.upgrade() {
                    let json = serde_json::to_string(&message).unwrap();
                    log::debug!("Send message {}", json);
                    if let Err(err) = sender.borrow_mut().send(WsMessage::Text(json)).await {
                        log::error!("Failed sending message: {:?}", err);
                    }
                }
            });
        }
    }
}
