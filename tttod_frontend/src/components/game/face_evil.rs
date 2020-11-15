use super::{
    ChallengeResultDialog, CharacterViewer, FinalChallengeDialog, OfferFinalChallenge, PlayerList,
};
use crate::{
    components::{Icon, ModalDialog},
    IconName,
};
use std::collections::{HashMap, HashSet};
use tttod_data::{Challenge, ChallengeResult, Condition, MentalCondition, Player};
use uuid::Uuid;
use ybc::{HeaderSize, TileCtx, TileSize};
use yew::prelude::*;

pub struct FaceEvil {
    link: ComponentLink<Self>,
    props: Props,
    dismissed_gm_modal: bool,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub player_id: Uuid,
    pub players: HashMap<Uuid, Player>,
    pub gms: HashSet<Uuid>,
    pub successes: usize,
    pub target_successes: usize,
    pub evil_state: EvilState,
    pub remaining_clues: Vec<String>,
    pub accept_challenge: yew::Callback<()>,
    pub reject_challenge: yew::Callback<()>,
    pub offer_challenge: yew::Callback<(Challenge, usize)>,
    pub use_artifact: yew::Callback<()>,
    pub take_wound: yew::Callback<()>,
    pub accept_fate: yew::Callback<()>,
}

pub enum Msg {
    DismissGMModal,
}

impl Component for FaceEvil {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            dismissed_gm_modal: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DismissGMModal => {
                self.dismissed_gm_modal = true;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let player = self.props.players.get(&self.props.player_id);
        let dismiss_modal = self.link.callback(|_| Msg::DismissGMModal);
        let is_gm = self.props.gms.contains(&self.props.player_id);
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
                            {"The Final Challenge"}
                        </ybc::Title>
                    </ybc::Tile>
                    <ybc::Tile classes="button-with-player-list" ctx=TileCtx::Child size=TileSize::Three>
                        <PlayerList player_id=self.props.player_id players=&self.props.players/>
                    </ybc::Tile>
                </ybc::Tile>
                <ybc::Tile vertical=false ctx=TileCtx::Parent>
                    <ybc::Tile classes="pt-4" vertical=true ctx=TileCtx::Child size=TileSize::Four>
                        <ybc::Box classes="p-1">
                            <ybc::Table classes="success-table" fullwidth=true>
                                <tbody>
                                    <tr><td class="success-table-label"><label class="label">{"Successes:"}</label></td><td class="success-table-progress"><ybc::Progress classes="is-primary" max={ self.props.target_successes as f32 } value={ self.props.successes as f32 }/></td><td class="success-table-summary">{self.props.successes}{"/"}{self.props.target_successes}</td></tr>
                                </tbody>
                            </ybc::Table>
                        </ybc::Box>
                    </ybc::Tile>
                    <ybc::Tile vertical=true ctx=TileCtx::Child size=TileSize::Eight>
                        <ybc::Box classes="m-4">
                            <ybc::Title size=HeaderSize::Is5>{"Remaining Secrets"}</ybc::Title>
                            <ol>
                                {
                                    for self.props.remaining_clues.iter().map(|clue| {
                                        html! {
                                            <li>{clue}</li>
                                        }
                                    })
                                }
                            </ol>
                        </ybc::Box>
                    </ybc::Tile>
                </ybc::Tile>
                <ybc::Tile vertical=false classes="is-flex-wrap-wrap" ctx=TileCtx::Parent size=TileSize::Twelve>
                    {
                        if is_gm {
                            self.props.players.iter().filter(|(&player_id, player)| {
                                player_id != self.props.player_id && player.condition != Condition::Dead && player.mental_condition != MentalCondition::Possessed
                            }).map(|(player_id, player)| {
                                let offer_challenge_callback = self.props.offer_challenge.clone();
                                html! {
                                    <ybc::Tile vertical=true ctx=TileCtx::Child size=TileSize::Six>
                                        <CharacterViewer classes="m-2" player=player.clone() header={
                                            if self.props.evil_state.challenge.is_none() {
                                                html! {
                                                    <FinalChallengeDialog remaining_clues=self.props.remaining_clues.clone() player_id=player_id player=player offer_challenge=offer_challenge_callback/>
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
                            <ModalDialog id="gm-notification" is_active=!self.dismissed_gm_modal close_callback=dismiss_modal.reform(|_| ()) title={
                                if self.props.gms.len() > 1 {
                                    format!("You Are One of the {} Game Masters For the Final Battle!", self.props.gms.len())
                                } else {
                                    "You Are the Game Master For the Final Battle!".to_owned()
                                }
                             } body={
                                html! {
                                    <>
                                        <ybc::Box classes="has-background-primary-light">
                                            <ybc::Media>
                                                <ybc::MediaLeft>
                                                    <Icon classes="has-text-warning is-size-2" name=IconName::ExclamationCircle/>
                                                </ybc::MediaLeft>
                                                <ybc::MediaContent>
                                                    <ybc::Title size=HeaderSize::Is5>{"Use These Secrets to Build the Final Room"}</ybc::Title>
                                                    <ol>
                                                        {
                                                            for self.props.remaining_clues.iter().map(|clue| {
                                                                html! {
                                                                    <li>{clue}</li>
                                                                }
                                                            })
                                                        }
                                                    </ol>
                                                </ybc::MediaContent>
                                            </ybc::Media>
                                        </ybc::Box>
                                        <p class="block">
                                            {"Once every player has been GM, the archeologists enter one final room. Here, in the in heart of the temple, \
                                            the ancient evil awakens, ready to end the world as we know it."}
                                        </p>
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
                                        <ybc::Button onclick=dismiss_modal.reform(|_| ())><Icon classes="icon" name=IconName::Gopuram/><span>{"The Final Room is Ready!"}</span></ybc::Button>
                                    </>
                                }
                            }/>
                        }
                    } else if let Some(player) = player {
                        let accept_challenge_callback = self.props.accept_challenge.clone();
                        let reject_challenge_callback = self.props.reject_challenge.clone();
                        if let Some((challenge, clue_idx)) = self.props.evil_state.challenge.as_ref() {
                            if *clue_idx < self.props.remaining_clues.len() {
                                let clue = self.props.remaining_clues[*clue_idx].clone();
                                html! {
                                    <>
                                        {
                                            if self.props.evil_state.challenge_result.is_none() {
                                                html! {
                                                    <OfferFinalChallenge clue=clue challenge=challenge player=player.clone() accept_challenge=accept_challenge_callback reject_challenge=reject_challenge_callback/>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                        <ChallengeResultDialog
                                            challenge_result=self.props.evil_state.challenge_result.clone()
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
                        } else {
                            html! {}
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
pub struct EvilState {
    pub challenge: Option<(Challenge, usize)>,
    pub challenge_result: Option<ChallengeResult>,
}
