use super::{CharacterViewer, PlayerList};
use std::collections::{HashMap, HashSet};
use tttod_data::{Challenge, ChallengeResult, Player};
use uuid::Uuid;
use ybc::{HeaderSize, TileCtx, TileSize};
use yew::prelude::*;

pub struct FaceEvil {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub player_id: Uuid,
    pub players: HashMap<Uuid, Player>,
    pub gms: HashSet<Uuid>,
    pub successes: usize,
    pub remaining_clues: Vec<String>,
    pub accept_challenge: yew::Callback<()>,
    pub reject_challenge: yew::Callback<()>,
    pub offer_challenge: yew::Callback<(Challenge, usize)>,
    pub use_artifact: yew::Callback<()>,
    pub take_wound: yew::Callback<()>,
    pub accept_fate: yew::Callback<()>,
}

pub enum Msg {}

impl Component for FaceEvil {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // match msg {
        // }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let player = self.props.players.get(&self.props.player_id);
        html! {}
    }
}
