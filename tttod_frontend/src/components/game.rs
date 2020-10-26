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
mod character_viewer;
pub use character_viewer::CharacterViewer;
mod room;
pub use room::{Room, RoomState};
mod challenge_dialog;
pub use challenge_dialog::ChallengeDialog;
mod final_challenge_dialog;
pub use final_challenge_dialog::FinalChallengeDialog;
mod offer_challenge;
pub use offer_challenge::OfferChallenge;
mod offer_final_challenge;
pub use offer_final_challenge::OfferFinalChallenge;
mod challenge_result;
pub use challenge_result::ChallengeResultDialog;
mod face_evil;
pub use face_evil::{EvilState, FaceEvil};
mod failure;
pub use failure::Failure;
mod success;
pub use success::Success;

use super::root::AppRoute;
use futures::{
    sink::SinkExt,
    stream::{SplitSink, StreamExt},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use tttod_data::{
    Challenge, ChallengeResult, ClientToServerMessage, GameState, Player, PlayerStats,
    ServerToClientMessage,
};
use uuid::Uuid;
use wasm_bindgen::{closure::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use ws_stream_wasm::{WsMessage, WsMeta, WsStream};
use ybc::{HeaderSize, TileCtx};
use yew::prelude::*;
use yew_router::{
    agent::RouteRequest::ChangeRoute,
    prelude::{Route, RouteAgentDispatcher},
};

pub struct Game {
    link: ComponentLink<Self>,
    props: Props,
    state: GameState,
    player_id: Uuid,
    websocket: Option<(WsMeta, Rc<RefCell<SplitSink<WsStream, WsMessage>>>)>,
    players: HashMap<Uuid, Player>,
    questions: Vec<(String, String)>,
    challenge_result: Option<ChallengeResult>,
    clue: Option<String>,
    game_over: bool,
    router: RouteAgentDispatcher,
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
    RejectSecret,
    OfferChallenge(Challenge),
    OfferChallengeFinal(Challenge, usize),
    AcceptChallenge,
    RejectChallenge,
    UseArtifact,
    TakeWound,
    AcceptFate,
    EndGame,
    // DEBUGGING ONLY
    FakeFailure,
    FakeSuccess,
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
            state: GameState::PlayerSelection {
                player_kick_votes: HashMap::new(),
            },
            player_id,
            websocket: None,
            players: HashMap::new(),
            questions: Vec::new(),
            challenge_result: None,
            clue: None,
            game_over: false,
            router: RouteAgentDispatcher::new(),
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
            Msg::OfferChallenge(challenge) => {
                self.send_message(ClientToServerMessage::OfferChallenge { challenge });
                false
            }
            Msg::OfferChallengeFinal(challenge, clue_idx) => {
                self.send_message(ClientToServerMessage::OfferChallengeFinal {
                    challenge,
                    clue_idx,
                });
                false
            }
            Msg::AcceptChallenge => {
                self.send_message(ClientToServerMessage::ChallengeAccepted);
                false
            }
            Msg::RejectChallenge => {
                self.send_message(ClientToServerMessage::ChallengeRejected);
                false
            }
            Msg::UseArtifact => {
                self.send_message(ClientToServerMessage::UseArtifact);
                self.challenge_result = None;
                false
            }
            Msg::TakeWound => {
                self.send_message(ClientToServerMessage::TakeWound);
                self.challenge_result = None;
                false
            }
            Msg::AcceptFate => {
                self.send_message(ClientToServerMessage::AcceptFate);
                self.challenge_result = None;
                false
            }
            Msg::FakeSuccess => {
                self.send_message(ClientToServerMessage::FakeSuccess);
                false
            }
            Msg::FakeFailure => {
                self.send_message(ClientToServerMessage::FakeFailure);
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
                    ServerToClientMessage::PushState {
                        players,
                        game_state,
                    } => {
                        if let (
                            GameState::Room {
                                room_idx: old_room, ..
                            },
                            GameState::Room {
                                room_idx: new_room, ..
                            },
                        ) = (&self.state, &game_state)
                        {
                            if old_room != new_room {
                                self.challenge_result = None;
                            }
                        } else if let (GameState::Room { .. }, GameState::FinalBattle { .. }) =
                            (&self.state, &game_state)
                        {
                            self.challenge_result = None;
                        } else if !matches!(game_state, GameState::Room { .. })
                            && !matches!(game_state, GameState::FinalBattle { .. })
                        {
                            self.challenge_result = None;
                        }

                        if matches!(game_state, GameState::Victory)
                            || matches!(game_state, GameState::Failure)
                        {
                            self.game_over = true;
                            if let Some((meta, _)) = self.websocket.take() {
                                meta.close();
                            }
                        }

                        self.state = game_state;
                        self.players = players;
                        true
                    }
                    ServerToClientMessage::Questions { questions } => {
                        self.questions = questions
                            .into_iter()
                            .map(|(q, a)| (q, a.unwrap_or_default()))
                            .collect();
                        true
                    }
                    ServerToClientMessage::PushClue { clue } => {
                        self.clue = Some(clue);
                        true
                    }
                    ServerToClientMessage::AbortedChallenge => {
                        self.challenge_result = None;
                        true
                    }
                    ServerToClientMessage::ChallengeResult(results) => {
                        self.challenge_result = Some(results);
                        true
                    }
                    _ => false,
                }
            }
            Msg::RejectSecret => {
                self.send_message(ClientToServerMessage::RejectClue);
                false
            }
            Msg::WebsocketClosed => {
                if !self.game_over {
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
                }
                false
            }
            Msg::ConnectWebsocket => {
                self.connect_websocket();
                false
            }
            Msg::EndGame => {
                self.router.send(ChangeRoute(Route::from(AppRoute::Index)));
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
        let reject_secret_callback = self.link.callback(|_| Msg::RejectSecret);
        let accept_challenge_callback = self.link.callback(|_| Msg::AcceptChallenge);
        let reject_challenge_callback = self.link.callback(|_| Msg::RejectChallenge);
        let use_artifact_callback = self.link.callback(|_| Msg::UseArtifact);
        let take_wound_callback = self.link.callback(|_| Msg::TakeWound);
        let accept_fate_callback = self.link.callback(|_| Msg::AcceptFate);

        let fake_failure_callback = self.link.callback(|_| Msg::FakeFailure);
        let fake_success_callback = self.link.callback(|_| Msg::FakeSuccess);

        html! {
            <ybc::Tile vertical=false ctx=TileCtx::Ancestor>
            {
                if self.websocket.is_some() || self.game_over {
                    log::debug!("state = {:?}", self.state);
                    match &self.state {
                        GameState::PlayerSelection { player_kick_votes } => {
                            html! {
                                <Lobby set_name=set_name_callback set_ready=set_ready_callback vote_kick=vote_kick_callback player_id=self.player_id players=self.players.clone() player_kick_votes=player_kick_votes.clone()/>
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
                        GameState::Room { room_idx, gm, successes, failures, known_clues, challenge } => {
                            let offer_challenge_callback = self.link.callback(Msg::OfferChallenge);
                            let room_state = RoomState {
                                challenge: challenge.clone(),
                                challenge_result: self.challenge_result.clone(),
                                clue: self.clue.clone(),
                            };
                            html! {
                                <Room
                                    player_id=self.player_id
                                    players=self.players.clone()
                                    room_idx=room_idx
                                    gm=gm
                                    successes=successes
                                    failures=failures
                                    state=room_state
                                    known_clues=known_clues.clone()
                                    reject_secret=reject_secret_callback
                                    offer_challenge=offer_challenge_callback
                                    accept_challenge=accept_challenge_callback
                                    reject_challenge=reject_challenge_callback
                                    use_artifact=use_artifact_callback
                                    take_wound=take_wound_callback
                                    accept_fate=accept_fate_callback
                                />
                            }
                        }
                        GameState::FinalBattle {
                            remaining_clues,
                            gms,
                            successes,
                            target_successes,
                            challenge,
                            chosen_clue,
                        } => {
                            let offer_challenge_final_callback = self.link.callback(|(challenge, clue_idx)| Msg::OfferChallengeFinal(challenge, clue_idx));
                            let evil_state = EvilState {
                                challenge: challenge.as_ref().map(|challenge| (challenge.clone(), chosen_clue.unwrap_or(0))),
                                challenge_result: self.challenge_result.clone(),
                            };
                            html! {
                                <FaceEvil
                                    player_id=self.player_id
                                    players=self.players.clone()
                                    gms=gms
                                    successes=successes
                                    target_successes=target_successes
                                    evil_state=evil_state
                                    remaining_clues=remaining_clues
                                    offer_challenge=offer_challenge_final_callback
                                    accept_challenge=accept_challenge_callback
                                    reject_challenge=reject_challenge_callback
                                    use_artifact=use_artifact_callback
                                    take_wound=take_wound_callback
                                    accept_fate=accept_fate_callback
                                />
                            }
                        }
                        GameState::Victory => {
                            let end_game_callback = self.link.callback(|_| Msg::EndGame);
                            html! {
                                <Success set_ready=end_game_callback/>
                            }
                        }
                        GameState::Failure => {
                            let end_game_callback = self.link.callback(|_| Msg::EndGame);
                            html! {
                                <Failure set_ready=end_game_callback/>
                            }
                        }
                    }
                } else {
                    html! {
                        <ybc::Title size=HeaderSize::Is4>{"Connecting to server…"}</ybc::Title>
                    }
                }
            }
            <ybc::Button onclick=fake_failure_callback>{"Fake Failure"}</ybc::Button>
            <ybc::Button onclick=fake_success_callback>{"Fake Success"}</ybc::Button>
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
                        log::error!("Unknown binary message received");
                    }
                }
                log::warn!("Websocket connection lost!");
                link.send_message(Msg::WebsocketClosed);
            } else {
                log::warn!("Unable to establish Websocket connection!");
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
