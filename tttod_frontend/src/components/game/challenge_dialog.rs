use tttod_data::Player;
use uuid::Uuid;
use ybc::{TileCtx, TileSize};
use yew::prelude::*;

pub struct ChallengeDialog {
    link: ComponentLink<Self>,
    props: Props,
    modal_bridge: yew::agent::Dispatcher<ybc::ModalCloser>,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub player_id: Uuid,
    pub player: Player,
    pub offer_challenge: Callback<()>,
}

pub enum Msg {
    OfferChallenge,
    Abort,
}

impl Component for ChallengeDialog {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            modal_bridge: ybc::ModalCloser::dispatcher(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OfferChallenge => {
                self.props.offer_challenge.emit(());
                self.modal_bridge.send(ybc::ModalCloseMsg(format!(
                    "offer-challenge-{}",
                    self.props.player_id
                )));
                true
            }
            Msg::Abort => {
                self.modal_bridge.send(ybc::ModalCloseMsg(format!(
                    "offer-challenge-{}",
                    self.props.player_id
                )));
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let offer_challenge_callback = self.link.callback(|_| Msg::OfferChallenge);
        let abort_challenge_callback = self.link.callback(|_| Msg::Abort);
        let update_speciality_applies_callback = yew::Callback::noop();
        let update_reputation_applies_callback = yew::Callback::noop();
        let update_attribute_callback = yew::Callback::noop();
        let stats = self.props.player.stats.as_ref().unwrap();
        let player_id = self.props.player_id;
        html! {
            <ybc::ModalCard id={format!("offer-challenge-{}", player_id)} trigger={
                html! {
                    <ybc::Button classes="mr-2">{"Challenge"}</ybc::Button>
                }
            } title=format!("Challenge Dr. {} (PhD)", stats.name) body={
                html! {
                    <>
                        <div class="block is-size-5">
                            {"The player has to argue how these elements apply to the challenge:"}
                            <div class="control is-size-5">
                                <ybc::Checkbox name="speciality_applies" checked=false update=update_speciality_applies_callback>
                                    {" The speciality of "}
                                    <span class="has-text-weight-bold">
                                        {format!("{}", stats.speciality)}
                                    </span>
                                    {" applies."}
                                </ybc::Checkbox>
                            </div>
                            <div class="control is-size-5">
                                <ybc::Checkbox name="reputation_applies" checked=false update=update_reputation_applies_callback>
                                    {" The character is living up to the reputation of "}
                                    <span class="has-text-weight-bold">
                                        {format!("{}", stats.reputation)}
                                    </span>
                                    {"."}
                                </ybc::Checkbox>
                            </div>
                        </div>
                        <ybc::Tile vertical=false>
                            <ybc::Tile ctx=TileCtx::Child size=TileSize::Four>
                                <ybc::Card classes="attribute-card">
                                    <ybc::CardHeader>
                                        <ybc::Radio classes="card-header-title is-size-5" name=format!("attribute-{}", player_id) value="heroic" checked_value=Some("heroic") update=update_attribute_callback.clone()>
                                            <span class="ml-2">{"Heroic"}</span>
                                        </ybc::Radio>
                                    </ybc::CardHeader>
                                    <div class="card-content">
                                        <ybc::Content>
                                            {"Brave, dramatic, powerful, physical, protecting others, leap into action, daredevil."}
                                        </ybc::Content>
                                    </div>
                                </ybc::Card>
                            </ybc::Tile>
                            <ybc::Tile ctx=TileCtx::Child size=TileSize::Four>
                                <ybc::Card classes="attribute-card">
                                    <ybc::CardHeader>
                                        <ybc::Radio classes="card-header-title is-size-5" name=format!("attribute-{}", player_id) value="booksmart" checked_value=Some("heroic") update=update_attribute_callback.clone()>
                                            <span class="ml-2">{"Booksmart"}</span>
                                        </ybc::Radio>
                                    </ybc::CardHeader>
                                    <div class="card-content">
                                        <ybc::Content>
                                            {"Uncovering, deciphering, investigating, revealing, deducing, using history and knowledge."}
                                        </ybc::Content>
                                    </div>
                                </ybc::Card>
                            </ybc::Tile>
                            <ybc::Tile ctx=TileCtx::Child size=TileSize::Four>
                                <ybc::Card classes="attribute-card">
                                    <ybc::CardHeader>
                                        <ybc::Radio classes="card-header-title is-size-5" name=format!("attribute-{}", player_id) value="streetwise" checked_value=Some("heroic") update=update_attribute_callback.clone()>
                                            <span class="ml-2">{"Streetwise"}</span>
                                        </ybc::Radio>
                                    </ybc::CardHeader>
                                    <div class="card-content">
                                        <ybc::Content>
                                            {"Cunning, outsmarting, fast-talking, quick thinking, fast reflexes, dodging, acrobatics."}
                                        </ybc::Content>
                                    </div>
                                </ybc::Card>
                            </ybc::Tile>
                        </ybc::Tile>
                    </>
                }
            } footer={
                html! {
                    <>
                        <ybc::Button onclick=abort_challenge_callback>{"Abort"}</ybc::Button>
                        <ybc::Button classes="has-background-danger" onclick=offer_challenge_callback>{"Offer Challenge"}</ybc::Button>
                    </>
                }
            }/>
        }
    }
}
