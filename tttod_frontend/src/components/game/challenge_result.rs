use crate::{components::Icon, IconName};
use tttod_data::{ArtifactBoon, Attribute, ChallengeResult, MentalCondition, Player};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use ybc::{TileCtx, TileSize};
use yew::prelude::*;

pub struct ChallengeResultDialog {
    link: ComponentLink<Self>,
    props: Props,
    modal_bridge: yew::agent::Dispatcher<ybc::ModalCloser>,
    show_dialog: NodeRef,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub player: Player,
    pub challenge_result: Option<ChallengeResult>,
}

pub enum Msg {
    AcceptChallenge,
    Abort,
}

impl Component for ChallengeResultDialog {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            modal_bridge: ybc::ModalCloser::dispatcher(),
            show_dialog: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // match msg {
        // }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.challenge_result.is_none() && props.challenge_result.is_some() {
            if let Some(show_dialog) = self.show_dialog.get() {
                show_dialog.unchecked_ref::<web_sys::HtmlElement>().click();
            }
        } else if self.props.challenge_result.is_some() && props.challenge_result.is_none() {
            self.modal_bridge
                .send(ybc::ModalCloseMsg("challenge-result".to_owned()));
        }
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <ybc::ModalCard id="challenge-result" trigger={
                html! {
                    <div class="is-invisible" ref=self.show_dialog.clone()></div>
                }
            } title="Your Challenge Result" body={
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
                                                        "The Ancient Evil is also trying to erode your will! You can use your artifact to attempt to avoid this."
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
                                                {"You can take a wound to turn it into a success. You are curently "}
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
                            <ybc::Button>{"Use Artifact"}</ybc::Button>
                        });
                    }
                    if challenge_result.success {
                        buttons.push(html! {
                            <ybc::Button classes="is-primary">{"Take Success"}</ybc::Button>
                        });
                    } else {
                        buttons.push(html! {
                            <ybc::Button classes="is-danger">{"Take Wound"}</ybc::Button>
                        });
                        buttons.push(html! {
                            <ybc::Button classes="is-danger">{"Accept Failure"}</ybc::Button>
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
