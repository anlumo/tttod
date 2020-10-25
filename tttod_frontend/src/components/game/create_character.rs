use super::PlayerList;
use crate::{components::Icon, IconName};
use std::collections::HashMap;
use tttod_data::{ArtifactBoon, Attribute, Player, PlayerStats, Reputation, Speciality};
use uuid::Uuid;
use ybc::{HeaderSize, TileCtx, TileSize};
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
    UpdateOtherSpeciality(String),
    UpdateReputation(String),
    UpdateOtherReputation(String),
    UpdateAttributes(String),
    UpdateArtifactName(String),
    UpdateArtifactOrigin(String),
    UpdateArtifactBoon(ArtifactBoon),
    Ready,
}

impl Component for CreateCharacter {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let loading = props
            .players
            .get(&props.player_id)
            .map(|player| player.ready)
            .unwrap_or(false);
        Self {
            link,
            props,
            loading,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Ready => {
                self.loading = true;
                self.props.set_ready.emit(());
                return true;
            }
            Msg::UpdateName(name) => {
                let mut stats = self.props.stats.clone();
                stats.name = name;
                self.props.set_character.emit(stats);
            }
            Msg::UpdateSpeciality(speciality) => {
                let mut stats = self.props.stats.clone();
                stats.speciality = match speciality.as_str() {
                    "Religion" => Speciality::Religion,
                    "Linguistics" => Speciality::Linguistics,
                    "Architecture" => Speciality::Architecture,
                    "War and Weaponry" => Speciality::WarAndWeaponry,
                    "Gems and Metals" => Speciality::GemsAndMetals,
                    "Secret Signs / Symbols" => Speciality::SecretSignsSymbols,
                    "Osteology" => Speciality::Osteology,
                    "Death and Burial" => Speciality::DeathAndBurial,
                    _ => Speciality::Other("".to_owned()),
                };
                self.props.set_character.emit(stats);
            }
            Msg::UpdateOtherSpeciality(speciality) => {
                let mut stats = self.props.stats.clone();
                stats.speciality = Speciality::Other(speciality);
                self.props.set_character.emit(stats);
            }
            Msg::UpdateReputation(reputation) => {
                let mut stats = self.props.stats.clone();
                stats.reputation = match reputation.as_str() {
                    "Ambitious" => Reputation::Ambitious,
                    "Genius" => Reputation::Genius,
                    "Ruthless" => Reputation::Ruthless,
                    "Senile" => Reputation::Senile,
                    "Mad Scientist" => Reputation::MadScientist,
                    "Born Leader" => Reputation::BornLeader,
                    "Rulebreaker" => Reputation::Rulebreaker,
                    "Obsessive" => Reputation::Obsessive,
                    _ => Reputation::Other("".to_owned()),
                };
                self.props.set_character.emit(stats);
            }
            Msg::UpdateOtherReputation(reputation) => {
                let mut stats = self.props.stats.clone();
                stats.reputation = Reputation::Other(reputation);
                self.props.set_character.emit(stats);
            }
            Msg::UpdateAttributes(code) => {
                let mut stats = self.props.stats.clone();
                stats.attributes = match code.as_str() {
                    "311" => [
                        (Attribute::Heroic, 3),
                        (Attribute::Booksmart, 1),
                        (Attribute::Streetwise, 1),
                    ],
                    "131" => [
                        (Attribute::Heroic, 1),
                        (Attribute::Booksmart, 3),
                        (Attribute::Streetwise, 1),
                    ],
                    "113" => [
                        (Attribute::Heroic, 1),
                        (Attribute::Booksmart, 1),
                        (Attribute::Streetwise, 3),
                    ],
                    "221" => [
                        (Attribute::Heroic, 2),
                        (Attribute::Booksmart, 2),
                        (Attribute::Streetwise, 1),
                    ],
                    "212" => [
                        (Attribute::Heroic, 2),
                        (Attribute::Booksmart, 1),
                        (Attribute::Streetwise, 2),
                    ],
                    "122" => [
                        (Attribute::Heroic, 1),
                        (Attribute::Booksmart, 2),
                        (Attribute::Streetwise, 2),
                    ],
                    _ => return false,
                }
                .iter()
                .cloned()
                .collect();
                self.props.set_character.emit(stats);
            }
            Msg::UpdateArtifactName(name) => {
                let mut stats = self.props.stats.clone();
                stats.artifact_name = name;
                self.props.set_character.emit(stats);
            }
            Msg::UpdateArtifactOrigin(origin) => {
                let mut stats = self.props.stats.clone();
                stats.artifact_origin = origin;
                self.props.set_character.emit(stats);
            }
            Msg::UpdateArtifactBoon(boon) => {
                let mut stats = self.props.stats.clone();
                stats.artifact_boon = boon;
                self.props.set_character.emit(stats);
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if let Some(player) = props.players.get(&props.player_id) {
            if player.ready {
                self.loading = true;
            }
        }
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let ready_callback = self.link.callback(|_| Msg::Ready);
        let update_name_callback = self.link.callback(Msg::UpdateName);
        let update_speciality_callback = self.link.callback(Msg::UpdateSpeciality);
        let update_other_speciality_callback = self.link.callback(Msg::UpdateOtherSpeciality);
        let update_reputation_callback = self.link.callback(Msg::UpdateReputation);
        let update_other_reputation_callback = self.link.callback(Msg::UpdateOtherReputation);
        let update_attributes_callback = self.link.callback(Msg::UpdateAttributes);
        let update_artifact_name_callback = self.link.callback(Msg::UpdateArtifactName);
        let update_artifact_origin_callback = self.link.callback(Msg::UpdateArtifactOrigin);
        let update_artifact_boon_callback = self.link.callback(|key: String| {
            Msg::UpdateArtifactBoon(match key.as_str() {
                "1" => ArtifactBoon::RollWithPlusTwo,
                "2" => ArtifactBoon::SuccessOnFive,
                "3" => ArtifactBoon::SuccessOnDoubles,
                _ => ArtifactBoon::Reroll,
            })
        });
        let player = self.props.players.get(&self.props.player_id);
        let stats = 100
            * (self
                .props
                .stats
                .attributes
                .get(&Attribute::Heroic)
                .cloned()
                .unwrap_or(0) as usize)
            + 10 * (self
                .props
                .stats
                .attributes
                .get(&Attribute::Booksmart)
                .cloned()
                .unwrap_or(0) as usize)
            + (self
                .props
                .stats
                .attributes
                .get(&Attribute::Streetwise)
                .cloned()
                .unwrap_or(0) as usize);
        let speciality = &self.props.stats.speciality;
        let reputation = &self.props.stats.reputation;
        let invalid_stats = self.props.stats.name.is_empty()
            || self.props.stats.artifact_name.is_empty()
            || self.props.stats.artifact_origin.is_empty();
        html! {
            <ybc::Tile vertical=true ctx=TileCtx::Parent>
                <ybc::Tile vertical=false ctx=TileCtx::Parent>
                    <ybc::Tile ctx=TileCtx::Child size=TileSize::Nine>
                        <ybc::Title size=HeaderSize::Is1>{"Create Your Archeologist"}</ybc::Title>
                    </ybc::Tile>
                    <ybc::Tile classes="button-with-player-list" ctx=TileCtx::Child size=TileSize::Three>
                        <ybc::Button loading=self.loading disabled=invalid_stats onclick=ready_callback>{"Let's Go!"}</ybc::Button>
                        <PlayerList player_id=self.props.player_id players=&self.props.players/>
                    </ybc::Tile>
                </ybc::Tile>
                <ybc::Tile vertical=false ctx=TileCtx::Child>
                    <div class="block">
                        <div class="field is-horizontal">
                            <div class="field-label is-normal">
                                <label class="label">{"Name:"}</label>
                            </div>
                            <div class="field-body">
                                <div class="field">
                                    <p class="control character-name-input">
                                        <div>{"Dr. "}</div><ybc::Input disabled=self.loading name="character_name" update=update_name_callback value=self.props.stats.name.clone() placeholder="Archeologist name"/><div>{" (PhD)"}</div>
                                    </p>
                                </div>
                            </div>
                        </div>
                        <div class={ if self.loading { "field is-horizontal is-align-items-baseline" } else { "field is-horizontal" } }>
                            <div class="field-label is-normal">
                                <label class="label">{"Speciality:"}</label>
                            </div>
                            <div class="field-body">
                                <div class="field">
                                    <p class="control speciality">
                                        {
                                            if self.loading {
                                                html! { {format!("{}", speciality)} }
                                            } else {
                                                html! {
                                                    <>
                                                        <ybc::Select name="speciality" value="" update=update_speciality_callback>
                                                            <option selected={speciality == &Speciality::Religion}>{"Religion"}</option>
                                                            <option selected={speciality == &Speciality::Linguistics}>{"Linguistics"}</option>
                                                            <option selected={speciality == &Speciality::Architecture}>{"Architecture"}</option>
                                                            <option selected={speciality == &Speciality::WarAndWeaponry}>{"War and Weaponry"}</option>
                                                            <option selected={speciality == &Speciality::GemsAndMetals}>{"Gems and Metals"}</option>
                                                            <option selected={speciality == &Speciality::SecretSignsSymbols}>{"Secret Signs / Symbols"}</option>
                                                            <option selected={speciality == &Speciality::Osteology}>{"Osteology"}</option>
                                                            <option selected={speciality == &Speciality::DeathAndBurial}>{"Death and Burial"}</option>
                                                            <option selected={matches!(speciality, Speciality::Other(_))}>{"other…"}</option>
                                                        </ybc::Select>
                                                        {
                                                            if let Speciality::Other(speciality) = speciality {
                                                                html! {
                                                                    <ybc::Input disabled=self.loading name="other_speciality" update=update_other_speciality_callback value=speciality.clone() placeholder="which one?"/>
                                                                }
                                                            } else {
                                                                html! { <></> }
                                                            }
                                                        }
                                                    </>
                                                }
                                            }
                                        }
                                    </p>
                                </div>
                            </div>
                        </div>
                        <div class={ if self.loading { "field is-horizontal is-align-items-baseline" } else { "field is-horizontal" } }>
                            <div class="field-label is-normal">
                                <label class="label">{"Reputation: "}</label>
                            </div>
                            <div class="field-body">
                                <div class="field">
                                    <p class="control reputation">
                                        {
                                            if self.loading {
                                                html! { {format!("{}", reputation)} }
                                            } else {
                                                html! {
                                                    <>
                                                        <ybc::Select name="reputation" value="" update=update_reputation_callback>
                                                        <option selected={reputation == &Reputation::Ambitious}>{"Ambitious"}</option>
                                                        <option selected={reputation == &Reputation::Genius}>{"Genius"}</option>
                                                        <option selected={reputation == &Reputation::Ruthless}>{"Ruthless"}</option>
                                                        <option selected={reputation == &Reputation::Senile}>{"Senile"}</option>
                                                        <option selected={reputation == &Reputation::MadScientist}>{"Mad Scientist"}</option>
                                                        <option selected={reputation == &Reputation::BornLeader}>{"Born Leader"}</option>
                                                        <option selected={reputation == &Reputation::Rulebreaker}>{"Rulebreaker"}</option>
                                                        <option selected={reputation == &Reputation::Obsessive}>{"Obsessive"}</option>
                                                        <option selected={matches!(reputation, Reputation::Other(_))}>{"other…"}</option>
                                                        </ybc::Select>
                                                        {
                                                            if let Reputation::Other(reputation) = reputation {
                                                                html! {
                                                                    <ybc::Input disabled=self.loading name="other_reputation" update=update_other_reputation_callback value=reputation.clone() placeholder="which one?"/>
                                                                }
                                                            } else {
                                                                html! { <></> }
                                                            }
                                                        }
                                                    </>
                                                }
                                            }
                                        }
                                    </p>
                                </div>
                            </div>
                        </div>
                        <div class={ if self.loading { "field is-horizontal is-align-items-baseline" } else { "field is-horizontal" } }>
                            <div class="field-label is-normal">
                                <label class="label">{"I'm"}</label>
                            </div>
                            <div class="field-body">
                                <div class="field">
                                    <p class="control">
                                        {
                                            if self.loading {
                                                html! {
                                                    {
                                                        match stats {
                                                            311 => "Heroic",
                                                            131 => "Booksmart",
                                                            113 => "Streetwise",
                                                            221 => "Heroic and Booksmart",
                                                            212 => "Heroic and Streetwise",
                                                            122 => "Booksmart and Streetwise",
                                                            _ => "unknown",
                                                        }
                                                    }
                                                }
                                            } else {
                                                html! {
                                                    <ybc::Select name="attributes" value="" update=update_attributes_callback>
                                                        <option value="311" selected={stats == 311}>{"Heroic"}</option>
                                                        <option value="131" selected={stats == 131}>{"Booksmart"}</option>
                                                        <option value="113" selected={stats == 113}>{"Streetwise"}</option>
                                                        <option value="221" selected={stats == 221}>{"Heroic and Booksmart"}</option>
                                                        <option value="212" selected={stats == 212}>{"Heroic and Streetwise"}</option>
                                                        <option value="122" selected={stats == 122}>{"Booksmart and Streetwise"}</option>
                                                    </ybc::Select>
                                                }
                                            }
                                        }
                                        </p>
                                </div>
                            </div>
                        </div>
                    </div>
                    <ybc::Tile vertical=false>
                        <ybc::Tile ctx=TileCtx::Child size=TileSize::Four>
                            <ybc::Card classes="attribute-card">
                                <ybc::CardHeader><p class="card-header-title">{"Heroic"}</p></ybc::CardHeader>
                                <div class="card-content">
                                    <ybc::Content>
                                        {"Brave, dramatic, powerful, physical, protecting others, leap into action, daredevil."}
                                    </ybc::Content>
                                </div>
                            </ybc::Card>
                        </ybc::Tile>
                        <ybc::Tile ctx=TileCtx::Child size=TileSize::Four>
                            <ybc::Card classes="attribute-card">
                                <ybc::CardHeader><p class="card-header-title">{"Booksmart"}</p></ybc::CardHeader>
                                <div class="card-content">
                                    <ybc::Content>
                                        {"Uncovering, deciphering, investigating, revealing, deducing, using history and knowledge."}
                                    </ybc::Content>
                                </div>
                            </ybc::Card>
                        </ybc::Tile>
                        <ybc::Tile ctx=TileCtx::Child size=TileSize::Four>
                            <ybc::Card classes="attribute-card">
                                <ybc::CardHeader><p class="card-header-title">{"Streetwise"}</p></ybc::CardHeader>
                                <div class="card-content">
                                    <ybc::Content>
                                        {"Cunning, outsmarting, fast-talking, quick thinking, fast reflexes, dodging, acrobatics."}
                                    </ybc::Content>
                                </div>
                            </ybc::Card>
                        </ybc::Tile>
                    </ybc::Tile>
                    <ybc::Title size=HeaderSize::Is2>{"Artifact"}</ybc::Title>
                    <div class="field is-horizontal">
                        <div class="field-body">
                            <div class="field">
                                <p class="control create-artifact">
                                    <div>{"A(n)"}</div>
                                    <ybc::Input disabled=self.loading name="artifact_name" update=update_artifact_name_callback value=self.props.stats.artifact_name.clone() placeholder="Name"/>
                                    <div>{"discovered in"}</div>
                                    <ybc::Input disabled=self.loading name="artifact_origin" update=update_artifact_origin_callback value=self.props.stats.artifact_origin.clone() placeholder="Origin"/>
                                    <div>{"."}</div>
                                </p>
                            </div>
                        </div>
                    </div>
                    <div class={ if self.loading { "field is-horizontal is-align-items-baseline" } else { "field is-horizontal" } }>
                        <div class="field-label is-normal">
                            <label class="label">{"Artifact Boon:"}</label>
                        </div>
                        <div class="field-body">
                            <div class="field">
                                <p class="control">
                                    {
                                        if self.loading {
                                            html! {
                                                {
                                                    format!("{}", self.props.stats.artifact_boon)
                                                }
                                            }
                                        } else {
                                            html! {
                                                <ybc::Select name="attributes" value="" update=update_artifact_boon_callback>
                                                    <option value="0" selected={self.props.stats.artifact_boon == ArtifactBoon::Reroll}>{"Reroll"}</option>
                                                    <option value="1" selected={self.props.stats.artifact_boon == ArtifactBoon::RollWithPlusTwo}>{"Roll with +2 dice"}</option>
                                                    <option value="2" selected={self.props.stats.artifact_boon == ArtifactBoon::SuccessOnFive}>{"Success on 5+"}</option>
                                                    <option value="3" selected={self.props.stats.artifact_boon == ArtifactBoon::SuccessOnDoubles}>{"Success on doubles"}</option>
                                                </ybc::Select>
                                            }
                                        }
                                    }
                                </p>
                            </div>
                        </div>
                    </div>
                </ybc::Tile>
            </ybc::Tile>
        }
    }
}
