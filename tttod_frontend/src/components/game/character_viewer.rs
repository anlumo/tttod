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
                            <p>
                                {format!("I specialize in {}. I'm known for being {}.", stats.speciality, stats.reputation)}
                            </p>
                            <p>
                                {format!("I once found the {} in {}. ", stats.artifact_name, stats.artifact_origin)}
                                {
                                    match stats.artifact_boon {
                                        ArtifactBoon::Reroll => "It allows me to reroll once.",
                                        ArtifactBoon::RollWithPlusTwo => "It allows me to roll two additional dice once.",
                                        ArtifactBoon::SuccessOnFive => "It makes a roll succeed on a 5 once.",
                                        ArtifactBoon::SuccessOnDoubles => "It makes a roll succeed on a double once.",
                                    }
                                }
                            </p>
                        </ybc::Content>
                    </ybc::CardContent>
                    <ybc::CardFooter>
                        <div class="card-footer-item">
                            <Icon classes="stat-rating" name=Self::stat_to_icon(stats, Attribute::Heroic)/>
                                {" Heroic"}
                        </div>
                        <div class="card-footer-item">
                            <Icon classes="stat-rating" name=Self::stat_to_icon(stats, Attribute::Booksmart)/>
                            {" Booksmart"}
                        </div>
                        <div class="card-footer-item">
                            <Icon classes="stat-rating" name=Self::stat_to_icon(stats, Attribute::Streetwise)/>
                            {" Streetwise"}
                        </div>
                    </ybc::CardFooter>
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
