use super::{ChallengeDialog, ChallengeResultDialog, CharacterViewer, OfferChallenge, PlayerList};
use crate::{components::Icon, IconName};
use std::collections::HashMap;
use tttod_data::{
    Challenge, ChallengeResult, Condition, MentalCondition, Player, FAILURES_NEEDED,
    SUCCESSES_NEEDED,
};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use ybc::{HeaderSize, TileCtx, TileSize};
use yew::prelude::*;

pub struct Room {
    link: ComponentLink<Self>,
    props: Props,
    dismissed_gm_modal: bool,
    modal_bridge: yew::agent::Dispatcher<ybc::ModalCloser>,
    rejected_secret: Option<String>,
    show_gm_notification: NodeRef,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub player_id: Uuid,
    pub players: HashMap<Uuid, Player>,
    pub room_idx: usize,
    pub gm: Uuid,
    pub successes: usize,
    pub failures: usize,
    pub state: RoomState,
    pub known_clues: Vec<String>,
    pub reject_secret: yew::Callback<()>,
    pub accept_challenge: yew::Callback<()>,
    pub reject_challenge: yew::Callback<()>,
    pub offer_challenge: yew::Callback<Challenge>,
    pub use_artifact: yew::Callback<()>,
    pub take_wound: yew::Callback<()>,
    pub accept_fate: yew::Callback<()>,
}

pub enum Msg {
    DismissGMModal,
    RejectSecret,
}

impl Component for Room {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            dismissed_gm_modal: false,
            modal_bridge: ybc::ModalCloser::dispatcher(),
            rejected_secret: None,
            show_gm_notification: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DismissGMModal => {
                self.dismissed_gm_modal = true;
                self.modal_bridge
                    .send(ybc::ModalCloseMsg("gm-notification".to_owned()));
                true
            }
            Msg::RejectSecret => {
                self.rejected_secret = self.props.state.clue.clone();
                self.props.reject_secret.emit(());
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.rejected_secret.is_some() && props.state.clue != self.rejected_secret {
            self.rejected_secret = None;
        }
        if self.props.gm == self.props.player_id && !self.dismissed_gm_modal {
            if let Some(show) = self.show_gm_notification.get() {
                show.unchecked_ref::<web_sys::HtmlElement>().click();
            }
        }
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        log::debug!("self.dismissed_gm_modal = {:?}", self.dismissed_gm_modal);
        log::debug!("gm: {:?}", self.props.gm == self.props.player_id);
        let dismiss_modal = self.link.callback(|_| Msg::DismissGMModal);
        let reject_secret_handler = self.link.callback(|_| Msg::RejectSecret);
        let is_gm = self.props.gm == self.props.player_id;
        let player = self.props.players.get(&self.props.player_id);
        html! {
            <ybc::Tile vertical=true ctx=TileCtx::Parent>
                <ybc::Tile vertical=false ctx=TileCtx::Parent>
                    <ybc::Tile ctx=TileCtx::Child size=TileSize::Nine>
                        <ybc::Title size=HeaderSize::Is1>
                            {
                                if is_gm {
                                    html! {
                                        <span title="You are the GM here!">
                                            <Icon classes="mr-3 has-text-primary gm-icon" name=IconName::BookReader/>
                                        </span>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                            {format!("Room {} of {}", self.props.room_idx + 1, self.props.players.len())}
                        </ybc::Title>
                    </ybc::Tile>
                    <ybc::Tile classes="button-with-player-list" ctx=TileCtx::Child size=TileSize::Three>
                        <PlayerList player_id=self.props.player_id players=&self.props.players/>
                    </ybc::Tile>
                </ybc::Tile>
                {
                    if let Some(clue) = &self.props.state.clue {
                        html! {
                            <ybc::Tile vertical=false ctx=TileCtx::Child>
                                <ybc::Box classes="m-4 has-background-primary-light">
                                    <ybc::Title size=HeaderSize::Is5>{"Secret for This Room"}</ybc::Title>
                                    <p>{
                                        clue
                                    }</p>
                                </ybc::Box>
                            </ybc::Tile>
                        }
                    } else {
                        html! {}
                    }
                }
                <ybc::Tile vertical=false ctx=TileCtx::Parent>
                    <ybc::Tile vertical=true ctx=TileCtx::Child size=TileSize::Three>
                        <ybc::Table classes="success-table" fullwidth=true>
                            <tbody>
                                <tr><td class="success-table-label"><label class="label">{"Failures:"}</label></td><td class="success-table-progress"><ybc::Progress classes="is-danger" max={ FAILURES_NEEDED as f32 } value={ self.props.failures as f32 }/></td><td class="success-table-summary">{self.props.failures}{"/"}{FAILURES_NEEDED}</td></tr>
                                <tr><td class="success-table-label"><label class="label">{"Successes:"}</label></td><td class="success-table-progress"><ybc::Progress classes="is-primary" max={ SUCCESSES_NEEDED as f32 } value={ self.props.successes as f32 }/></td><td class="success-table-summary">{self.props.successes}{"/"}{SUCCESSES_NEEDED}</td></tr>
                            </tbody>
                        </ybc::Table>
                    </ybc::Tile>
                    {
                        if !self.props.known_clues.is_empty() {
                            html! {
                                <ybc::Tile vertical=true ctx=TileCtx::Child size=TileSize::Nine>
                                    <ybc::Box classes="m-4">
                                        <ybc::Title size=HeaderSize::Is5>{"Known Secrets"}</ybc::Title>
                                        <ul>
                                            {
                                                for self.props.known_clues.iter().map(|clue| {
                                                    html! {
                                                        <li>{clue}</li>
                                                    }
                                                })
                                            }
                                        </ul>
                                    </ybc::Box>
                                </ybc::Tile>
                            }
                        } else {
                            html! {}
                        }
                    }
                </ybc::Tile>
                <ybc::Tile vertical=false ctx=TileCtx::Parent size=TileSize::Twelve>
                    {
                        if is_gm {
                            self.props.players.iter().filter(|(&player_id, player)| {
                                player_id != self.props.player_id && player.condition != Condition::Dead && player.mental_condition != MentalCondition::Possessed
                            }).map(|(player_id, player)| {
                                let offer_challenge_callback = self.props.offer_challenge.clone();
                                html! {
                                    <ybc::Tile vertical=true ctx=TileCtx::Child size=TileSize::Six>
                                        <CharacterViewer classes="m-2" player=player.clone() header={
                                            if self.props.state.challenge.is_none() {
                                                html! {
                                                    <ChallengeDialog player_id=player_id player=player offer_challenge=offer_challenge_callback/>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }/>
                                    </ybc::Tile>
                                }
                            }).collect()
                        } else if let Some(player) = player {
                            html! {
                                <ybc::Tile vertical=true ctx=TileCtx::Child size=TileSize::Six>
                                    <CharacterViewer player=player.clone()/>
                                </ybc::Tile>
                            }
                        } else {
                            html! {}
                        }
                    }
                </ybc::Tile>
                {
                    if is_gm {
                        html! {
                            <ybc::ModalCard id="gm-notification" trigger={
                                html! {
                                    <div class="is-invisible" ref=self.show_gm_notification.clone()></div>
                                }
                            } title="You Are the Game Master Now!" body={
                                html! {
                                    <>
                                        <ybc::Box classes="has-background-primary-light">
                                            <ybc::Media>
                                                <ybc::MediaLeft>
                                                    <Icon classes="has-text-warning is-size-2" name=IconName::ExclamationCircle/>
                                                </ybc::MediaLeft>
                                                <ybc::MediaContent>
                                                    <ybc::Title size=HeaderSize::Is5>{"Use This Secret to Build Your Room"}</ybc::Title>
                                                    {
                                                        if let Some(clue) = &self.props.state.clue {
                                                            html! {
                                                                <p>{clue}</p>
                                                            }
                                                        } else {
                                                            html! {}
                                                        }
                                                    }
                                                </ybc::MediaContent>
                                            </ybc::Media>
                                        </ybc::Box>
                                        <ybc::Title size=HeaderSize::Is4>{"Help With Creating Rooms"}</ybc::Title>
                                        <p class="block">
                                            {"Use these for inspiration! Or pick three and combine with flair to create a chamber that reflects the secret shown above."}
                                        </p>
                                        <ybc::Title size=HeaderSize::Is5>{"Puzzles & Riddles"}</ybc::Title>
                                        <p class="block">
                                            {"Strange runic patterns, carefully arranged gems of power, statues \
                                            with rotating heads, movable dials, a chessboard floor, countless \
                                            levers, whispered rhymes sung by a thousand lipless mouths."}
                                        </p>
                                        <ybc::Title size=HeaderSize::Is5>{"Environmental Obstacles"}</ybc::Title>
                                        <p class="block">
                                            {"Spike pits, lava stream, walls closing in on each other, rapidly \
                                            rising water, narrow ledges, decaying or invisible bridges, unnatural \
                                            snow or sandstorms."}
                                        </p>
                                        <ybc::Title size=HeaderSize::Is5>{"Traps"}</ybc::Title>
                                        <p class="block">
                                            {"Flaming jets, poison darts, trapped chests, fake floors, cursed \
                                            altars, rolling boulders, deadly illusions, reverse or shifting \
                                            gravity, cursed magical items."}
                                        </p>
                                        <ybc::Title size=HeaderSize::Is5>{"Enemies"}</ybc::Title>
                                        <p class="block">
                                            {"Venomous snakes, roaming mummies, dark spirits, swarm of scarab \
                                            beetles or scorpions, Nazis, cult members, and of course the most \
                                            terrifying of all: evil archeologists."}
                                        </p>
                                    </>
                                }
                            } footer={
                                html! {
                                    <>
                                        {
                                            if self.props.room_idx > 0 {
                                                html! {
                                                    <ybc::Button classes="is-danger is-light" loading=self.rejected_secret.is_some() onclick=reject_secret_handler><Icon classes="icon" name=IconName::Times/><span>{"This secret conficts with established lore"}</span></ybc::Button>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                        <ybc::Button onclick=dismiss_modal.clone()><Icon classes="icon" name=IconName::Gopuram/><span>{"My room is ready!"}</span></ybc::Button>
                                    </>
                                }
                            }/>
                        }
                    } else if let Some(player) = player {
                        let accept_challenge_callback = self.props.accept_challenge.clone();
                        let reject_challenge_callback = self.props.reject_challenge.clone();
                        html! {
                            <>
                                {
                                    if self.props.state.challenge_result.is_none() {
                                        html! {
                                            <OfferChallenge challenge=self.props.state.challenge.clone() player=player.clone() accept_challenge=accept_challenge_callback reject_challenge=reject_challenge_callback/>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                                <ChallengeResultDialog
                                    challenge_result=self.props.state.challenge_result.clone()
                                    player=player.clone()
                                    use_artifact=self.props.use_artifact.clone()
                                    take_wound=self.props.take_wound.clone()
                                    accept_fate=self.props.accept_fate.clone()
                                />
                            </>
                        }
                    } else {
                        html! {}
                    }
                }
            </ybc::Tile>
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RoomState {
    pub challenge: Option<Challenge>,
    pub challenge_result: Option<ChallengeResult>,
    pub clue: Option<String>,
}
