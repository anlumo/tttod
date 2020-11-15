use crate::{components::Icon, IconName};
use tttod_data::{ArtifactBoon, Attribute, Player, PlayerStats};
use yew::prelude::*;

pub struct CharacterViewer {
    props: Props,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub player: Player,
    #[prop_or_default]
    pub header: Html,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_default]
    pub brief: bool,
}

impl Component for CharacterViewer {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let player = &self.props.player;
        if let Some(stats) = &player.stats {
            html! {
                <ybc::Card classes=self.props.classes.clone()>
                    <ybc::CardHeader classes="is-align-items-center">
                        <p class="card-header-title">
                            {format!("[{}] Dr. {} (PhD)", player.name, stats.name)}
                        </p>
                        {self.props.header.clone()}
                    </ybc::CardHeader>
                    <ybc::CardContent>
                        <ybc::Content tag="div">
                            {
                                if !self.props.brief {
                                    html! {
                                        <p>
                                            {"Physically "}
                                            <span class="has-text-weight-bold explanation" title="Hale ► Wounded ► Critical ► Dead">
                                                {format!("{}", player.condition)}
                                            </span>
                                            {". Mentally "}
                                            <span class="has-text-weight-bold explanation" title="Hale ► Resisted ► Possessed">
                                                {format!("{}", player.mental_condition)}
                                            </span>
                                            {"."}
                                        </p>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                            <p>
                                {"I specialize in "}
                                <span class="has-text-weight-bold">
                                    {format!("{}", stats.speciality)}
                                </span>
                                {". I'm known for being "}
                                <span class="has-text-weight-bold">
                                    {format!("{}", stats.reputation)}
                                </span>
                                {"."}
                            </p>
                            {
                                if player.artifact_used || self.props.brief {
                                    html! {}
                                } else {
                                    html! {
                                        <p>
                                            <Icon classes="icon" name=IconName::ChessQueen/>
                                            {"I once found the "}
                                            <span class="has-text-weight-bold">
                                                {stats.artifact_name.as_str()}
                                            </span>
                                            {" in "}
                                            <span class="has-text-weight-bold">
                                                {stats.artifact_origin.as_str()}
                                            </span>
                                            {". "}
                                            {
                                                match stats.artifact_boon {
                                                    ArtifactBoon::Reroll => "It allows me to reroll once.",
                                                    ArtifactBoon::RollWithPlusTwo => "It allows me to roll two additional dice once.",
                                                    ArtifactBoon::SuccessOnFive => "It makes a roll succeed on a 5 once.",
                                                    ArtifactBoon::SuccessOnDoubles => "It makes a roll succeed on a double once.",
                                                }
                                            }
                                        </p>
                                    }
                                }
                            }
                        </ybc::Content>
                    </ybc::CardContent>
                    {
                        if !self.props.brief {
                            html! {
                                <ybc::CardFooter>
                                    <ybc::Dropdown classes="card-footer-item is-up" button_classes="is-white" hoverable=true button_html={
                                        html! {
                                            <>
                                                <Icon classes="stat-rating" name=Self::stat_to_icon(stats, Attribute::Heroic)/>
                                                {" Heroic"}
                                            </>
                                        }
                                    }>
                                        <div class="dropdown-item">
                                            <p>
                                                { "Brave, dramatic, powerful, physical, protecting others, leap into action, daredevil." }
                                                {
                                                    if let Some(stat) = stats.attributes.get(&Attribute::Heroic) {
                                                        html! {
                                                            <p class="has-text-weight-bold">{ format!("{} dice", stat) }</p>
                                                        }
                                                    } else {
                                                        html! {}
                                                    }
                                                }
                                            </p>
                                        </div>
                                    </ybc::Dropdown>
                                    <ybc::Dropdown classes="card-footer-item is-up" button_classes="is-white" hoverable=true button_html={
                                        html! {
                                            <>
                                                <Icon classes="stat-rating" name=Self::stat_to_icon(stats, Attribute::Booksmart)/>
                                                {" Booksmart"}
                                            </>
                                        }
                                    }>
                                        <div class="dropdown-item">
                                            <p>
                                                { "Uncovering, deciphering, investigating, revealing, deducing, using history and knowledge." }
                                                {
                                                    if let Some(stat) = stats.attributes.get(&Attribute::Booksmart) {
                                                        html! {
                                                            <p class="has-text-weight-bold">{ format!("{} dice", stat) }</p>
                                                        }
                                                    } else {
                                                        html! {}
                                                    }
                                                }
                                            </p>
                                        </div>
                                    </ybc::Dropdown>
                                    <ybc::Dropdown classes="card-footer-item is-up" button_classes="is-white" hoverable=true button_html={
                                        html! {
                                            <>
                                                <Icon classes="stat-rating" name=Self::stat_to_icon(stats, Attribute::Streetwise)/>
                                                {" Streetwise"}
                                            </>
                                        }
                                    }>
                                        <div class="dropdown-item">
                                            <p>
                                                { "Cunning, outsmarting, fast-talking, quick thinking, fast reflexes, dodging, acrobatics." }
                                                {
                                                    if let Some(stat) = stats.attributes.get(&Attribute::Streetwise) {
                                                        html! {
                                                            <p class="has-text-weight-bold">{ format!("{} dice", stat) }</p>
                                                        }
                                                    } else {
                                                        html! {}
                                                    }
                                                }
                                            </p>
                                        </div>
                                    </ybc::Dropdown>
                                </ybc::CardFooter>
                            }
                        } else {
                            html! {}
                        }
                    }
                </ybc::Card>
            }
        } else {
            html! { <></> }
        }
    }
}

impl CharacterViewer {
    fn stat_to_icon(stats: &PlayerStats, attribute: Attribute) -> IconName {
        match stats.attributes.get(&attribute) {
            Some(3) => IconName::AngleDoubleUp,
            Some(2) => IconName::AngleUp,
            Some(1) => IconName::AngleDown,
            _ => IconName::Asterisk,
        }
    }
}
