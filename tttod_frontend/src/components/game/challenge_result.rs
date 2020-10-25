use crate::{
    components::{Icon, ModalDialog},
    IconName,
};
use tttod_data::{ArtifactBoon, ChallengeResult, MentalCondition, Player};
use yew::prelude::*;

pub struct ChallengeResultDialog {
    props: Props,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub player: Player,
    pub challenge_result: Option<ChallengeResult>,
    pub use_artifact: yew::Callback<()>,
    pub take_wound: yew::Callback<()>,
    pub accept_fate: yew::Callback<()>,
}

impl Component for ChallengeResultDialog {
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
            <ModalDialog id="challenge-result" is_active=self.props.challenge_result.is_some() title="Your Challenge Result" body={
                if let Some(challenge_result) = &self.props.challenge_result {
                    html! {
                        <>
                            <ybc::Box classes="dice-list has-background-primary-light">
                                {
                                    for challenge_result.rolls.iter().map(|die| {
                                        html! {
                                            <Icon name={
                                                match die {
                                                    1 => IconName::DiceOne,
                                                    2 => IconName::DiceTwo,
                                                    3 => IconName::DiceThree,
                                                    4 => IconName::DiceFour,
                                                    5 => IconName::DiceFive,
                                                    6 => IconName::DiceSix,
                                                    _ => IconName::DiceD6,
                                                }
                                            }/>
                                        }
                                    })
                                }
                            </ybc::Box>
                            <div class="block">
                                <p>
                                    {"This is a "}
                                    <span class="has-text-weight-bold">
                                        {
                                            if challenge_result.success {
                                                "success"
                                            } else {
                                                "failure"
                                            }
                                        }
                                    </span>
                                    {"!"}
                                </p>
                                {
                                    if challenge_result.possession {
                                        let stats = self.props.player.stats.as_ref().unwrap();
                                        html! {
                                            <p>
                                                {
                                                    if challenge_result.can_use_artifact && stats.artifact_boon == ArtifactBoon::Reroll {
                                                        if self.props.player.mental_condition == MentalCondition::Resisted {
                                                            "The Ancient Evil is also trying to erode your will! You can use your artifact to attempt to avoid this, or you get turned this time."
                                                        } else {
                                                            "The Ancient Evil is also trying to erode your will! You can use your artifact to attempt to avoid this."
                                                        }
                                                    } else if self.props.player.mental_condition == MentalCondition::Resisted {
                                                        "The Ancient Evil has you under their control! Reveal your true nature at will and mysteriously \
                                                        disappear. You will return to aid your new master in the final battle."
                                                    } else {
                                                        "The Ancient Evil is also trying to erode your will! However, you were able to resist this one time."
                                                    }
                                                }
                                            </p>
                                        }
                                    } else if challenge_result.can_use_artifact {
                                        let stats = self.props.player.stats.as_ref().unwrap();
                                        html! {
                                            <p>
                                                {"You can use your artifact to avoid this situation. "}
                                                {
                                                    match stats.artifact_boon {
                                                        ArtifactBoon::Reroll => "It allows you to reroll all dice.",
                                                        ArtifactBoon::RollWithPlusTwo => "It allows you to roll two additional dice.",
                                                        ArtifactBoon::SuccessOnFive => "It makes the roll a success on 5+.",
                                                        ArtifactBoon::SuccessOnDoubles => "It makes the roll a success on doubles.",
                                                    }
                                                }
                                            </p>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                                {
                                    if !challenge_result.success {
                                        html! {
                                            <p>
                                                {"You can take a wound to turn it into a success. You are currently "}
                                                <span class="has-text-weight-bold">
                                                    { format!("{}", self.props.player.condition) }
                                                </span>
                                                {". This wound would turn you "}
                                                <span class="has-text-weight-bold">
                                                    { format!("{}", self.props.player.condition.take_hit()) }
                                                </span>
                                                {"."}
                                            </p>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>
                        </>
                    }
                } else {
                    html! {}
                }
            } footer={
                if let Some(challenge_result) = &self.props.challenge_result {
                    let mut buttons = Vec::new();
                    if challenge_result.can_use_artifact {
                        buttons.push(html! {
                            <ybc::Button onclick=self.props.use_artifact.reform(|_| ())><Icon classes="icon" name=IconName::ChessQueen/><span>{"Use Artifact"}</span></ybc::Button>
                        });
                    }
                    if challenge_result.success {
                        buttons.push(html! {
                            <ybc::Button classes="is-primary" onclick=self.props.accept_fate.reform(|_| ())><Icon classes="icon" name=IconName::Child/><span>{"Take Success"}</span></ybc::Button>
                        });
                    } else {
                        buttons.push(html! {
                            <ybc::Button classes="is-danger" onclick=self.props.take_wound.reform(|_| ())><Icon classes="icon" name=IconName::Wheelchair/><span>{"Take Wound"}</span></ybc::Button>
                        });
                        buttons.push(html! {
                            <ybc::Button classes="is-danger" onclick=self.props.accept_fate.reform(|_| ())><Icon classes="icon" name=IconName::Dizzy/><span>{"Accept Failure"}</span></ybc::Button>
                        });
                    }
                    buttons.into_iter().collect()
                } else {
                    html! {}
                }
            }/>
        }
    }
}
