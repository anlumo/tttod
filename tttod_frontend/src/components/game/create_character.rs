use crate::{components::Icon, IconName};
use std::collections::HashMap;
use tttod_data::{Player, PlayerStats};
use uuid::Uuid;
use ybc::{HeaderSize, Size, TileCtx, TileSize};
use yew::prelude::*;

pub struct CreateCharacter {
    link: ComponentLink<Self>,
    props: Props,
    loading: bool,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub stats: PlayerStats,
    pub player_id: Uuid,
    pub players: HashMap<Uuid, Player>,
    pub set_character: Callback<PlayerStats>,
    pub set_ready: Callback<()>,
}

pub enum Msg {
    UpdateName(String),
    UpdateSpeciality(String),
    UpdateReputation(String),
    Ready,
}

impl Component for CreateCharacter {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            loading: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ready => {
                self.props.set_ready.emit(());
            }
            Msg::UpdateName(name) => {
                let mut stats = self.props.stats.clone();
                stats.name = name;
                self.props.set_character.emit(stats);
            }
            Msg::UpdateSpeciality(speciality) => {
                let mut stats = self.props.stats.clone();
                stats.speciality = speciality;
                self.props.set_character.emit(stats);
            }
            Msg::UpdateReputation(reputation) => {
                let mut stats = self.props.stats.clone();
                stats.reputation = reputation;
                self.props.set_character.emit(stats);
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let ready_callback = self.link.callback(|_| Msg::Ready);
        let update_name_callback = self.link.callback(Msg::UpdateName);
        let update_speciality_callback = self.link.callback(Msg::UpdateSpeciality);
        let update_reputation_callback = self.link.callback(Msg::UpdateReputation);
        html! {
            <ybc::Tile vertical=false ctx=TileCtx::Parent>
                <ybc::Tile vertical=false ctx=TileCtx::Child size=TileSize::Eight>
                    <ybc::Title size=HeaderSize::Is1>{"Create Your Archeologist"}</ybc::Title>
                    <p class="block">{"Dr. "}<ybc::Input disabled=self.loading name="character_name" update=update_name_callback value=self.props.stats.name.clone() placeholder="Archeologist name"/>{" (PhD)"}</p>
                    <p class="block">{"Speciality: "}
                        <ybc::Select name="speciality" value=self.props.stats.speciality.clone() update=update_speciality_callback loading=self.loading>
                            <option>{"Religion"}</option>
                            <option>{"Linguistics"}</option>
                            <option>{"Architecture"}</option>
                            <option>{"War and Weaponry"}</option>
                            <option>{"Gems and Metals"}</option>
                            <option>{"Secret Signs / Symbols"}</option>
                            <option>{"Osteology"}</option>
                            <option>{"Death and Burial"}</option>
                            <option>{"other…"}</option>
                        </ybc::Select>
                    </p>
                    <p class="block">{"Reputation: "}
                        <ybc::Select name="reputation" value=self.props.stats.reputation.clone() update=update_reputation_callback loading=self.loading>
                            <option>{"Ambitious"}</option>
                            <option>{"Genius"}</option>
                            <option>{"Ruthless"}</option>
                            <option>{"Senile"}</option>
                            <option>{"Mad Scientist"}</option>
                            <option>{"Born Leader"}</option>
                            <option>{"Rulebreaker"}</option>
                            <option>{"Obsessive"}</option>
                            <option>{"other…"}</option>
                        </ybc::Select>
                    </p>
                    <ybc::Title size=HeaderSize::Is4>{"Stats"}</ybc::Title>
                    <p>{"Heroic: Brave, dramatic, powerful, physical, protecting others, leap into action, daredevil."}</p>
                    <p>{"Booksmart: Uncovering, deciphering, investigating, revealing, deducing, using history and knowledge."}</p>
                    <p>{"Streetwise: Cunning, outsmarting, fast-talking, quick thinking, fast reflexes, dodging, acrobatics."}</p>
                </ybc::Tile>
                <ybc::Tile vertical=true ctx=TileCtx::Parent>
                    <ybc::Tile ctx=TileCtx::Child size=TileSize::Twelve>
                        <ybc::Button loading=self.loading disabled=self.props.stats.name.is_empty() onclick=ready_callback>{"Let's Go!"}</ybc::Button>
                    </ybc::Tile>
                    <ybc::Tile classes="box" ctx=TileCtx::Child>
                        <ybc::Title size=HeaderSize::Is4>{"Players"}</ybc::Title>
                        <ybc::Table striped=true narrow=true fullwidth=true>
                            <thead>
                                <tr><th></th><th class="name">{"Name"}</th></tr>
                            </thead>
                            <tbody>
                            {
                                for self.props.players.iter().map(move |(player_id, player)| {
                                    html! {
                                        <tr><td>{
                                            if player.ready {
                                                html! {
                                                    <Icon name=IconName::CheckCircle/>
                                                }
                                            } else {
                                                html! {
                                                    <Icon name=IconName::Hourglass/>
                                                }
                                            }
                                        }</td><td class="name">{&player.name}</td></tr>
                                    }
                                })
                            }
                            </tbody>
                        </ybc::Table>
                    </ybc::Tile>
                </ybc::Tile>
            </ybc::Tile>
        }
    }
}
