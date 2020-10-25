use crate::{
    components::{Icon, ModalDialog},
    IconName,
};
use tttod_data::{Attribute, Challenge, Player};
use yew::prelude::*;

pub struct OfferFinalChallenge {
    props: Props,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub player: Player,
    pub challenge: Option<Challenge>,
    pub clue: String,
    pub accept_challenge: Callback<()>,
    pub reject_challenge: Callback<()>,
}

impl Component for OfferFinalChallenge {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <ModalDialog id="offer-final-challenge" is_active=self.props.challenge.is_some() title="Challenge Received!" close_callback=self.props.reject_challenge.clone() body={
                if let Some(challenge) = &self.props.challenge {
                    let dice_count = *self
                        .props
                        .player
                        .stats
                        .as_ref()
                        .unwrap()
                        .attributes
                        .get(&challenge.attribute)
                        .unwrap()
                        + if challenge.speciality_applies { 1 } else { 0 }
                        + if challenge.reputation_applies { 1 } else { 0 };

                    html! {
                        <>
                            <div class="block">
                                {"You are using the following secret:"}
                                <p>{self.props.clue.as_str()}</p>
                            </div>
                            <div class="block">
                                <p>
                                    {"This challenge needs your "}
                                    <span class="has-text-weight-bold">
                                    {
                                        match challenge.attribute {
                                            Attribute::Heroic => "Heroic",
                                            Attribute::Booksmart => "Booksmart",
                                            Attribute::Streetwise => "Streetwise"
                                        }
                                    }
                                    </span>
                                    {" qualities!"}
                                </p>
                                {
                                    if challenge.speciality_applies {
                                        html! {
                                            <p>
                                                {"Your speciality of "}
                                                <span class="has-text-weight-bold">
                                                    { format!("{}", self.props.player.stats.as_ref().unwrap().speciality) }
                                                </span>
                                                {" applies."}
                                            </p>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                                {
                                    if challenge.reputation_applies {
                                        html! {
                                            <p>
                                                {"You would live up to your reputation of "}
                                                <span class="has-text-weight-bold">
                                                    { format!("{}", self.props.player.stats.as_ref().unwrap().reputation) }
                                                </span>
                                                {"."}
                                            </p>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                                <p>{format!("You would get {}d6 for this roll. At least one has to be a 6 to succeed.", dice_count)}</p>
                            </div>
                            <ybc::Box classes="dice-list has-background-primary-light">
                                {
                                    for (0..dice_count).map(|_| {
                                        html! {
                                            <Icon name=IconName::DiceD6/>
                                        }
                                    })
                                }
                            </ybc::Box>
                        </>
                    }
                } else {
                    html! {}
                }
            } footer={
                html! {
                    <>
                        <ybc::Button onclick=self.props.reject_challenge.reform(|_| ())><Icon classes="icon" name=IconName::Times/><span>{"Refuse"}</span></ybc::Button>
                        <ybc::Button classes="has-background-danger" onclick=self.props.accept_challenge.reform(|_| ())><Icon classes="icon" name=IconName::Dice/><span>{"Accept Challenge"}</span></ybc::Button>
                    </>
                }
            }/>
        }
    }
}
