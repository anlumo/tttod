use crate::{components::Icon, IconName};
use tttod_data::{Attribute, Challenge, Player};
use wasm_bindgen::JsCast;
use yew::prelude::*;

pub struct OfferFinalChallenge {
    link: ComponentLink<Self>,
    props: Props,
    modal_bridge: yew::agent::Dispatcher<ybc::ModalCloser>,
    show_dialog: NodeRef,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub player: Player,
    pub challenge: Option<Challenge>,
    pub clue: String,
    pub accept_challenge: Callback<()>,
    pub reject_challenge: Callback<()>,
}

pub enum Msg {
    AcceptChallenge,
    Abort,
}

impl Component for OfferFinalChallenge {
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
        match msg {
            Msg::AcceptChallenge => {
                self.modal_bridge
                    .send(ybc::ModalCloseMsg("offer-final-challenge".to_owned()));
                self.props.accept_challenge.emit(());
                true
            }
            Msg::Abort => {
                self.modal_bridge
                    .send(ybc::ModalCloseMsg("offer-final-challenge".to_owned()));
                self.props.reject_challenge.emit(());
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props.challenge.is_some() {
            if let Some(show_dialog) = self.show_dialog.get() {
                show_dialog.unchecked_ref::<web_sys::HtmlElement>().click();
            }
        } else if props.challenge.is_none() {
            self.modal_bridge
                .send(ybc::ModalCloseMsg("offer-final-challenge".to_owned()));
        }
        self.props = props;
        true
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render && self.props.challenge.is_some() {
            if let Some(show_dialog) = self.show_dialog.get() {
                show_dialog.unchecked_ref::<web_sys::HtmlElement>().click();
            }
        }
    }

    fn view(&self) -> Html {
        let accept_challenge_callback = self.link.callback(|_| Msg::AcceptChallenge);
        let abort_challenge_callback = self.link.callback(|_| Msg::Abort);

        html! {
            <ybc::ModalCard id="offer-final-challenge" trigger={
                html! {
                    <div class="is-invisible" ref=self.show_dialog.clone()></div>
                }
            } title="Challenge Received!" body={
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
                        <ybc::Button onclick=abort_challenge_callback><Icon classes="icon" name=IconName::Times/><span>{"Refuse"}</span></ybc::Button>
                        <ybc::Button classes="has-background-danger" onclick=accept_challenge_callback><Icon classes="icon" name=IconName::Dice/><span>{"Accept Challenge"}</span></ybc::Button>
                    </>
                }
            }/>
        }
    }
}
