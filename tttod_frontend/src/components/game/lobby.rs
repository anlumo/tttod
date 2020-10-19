use crate::{
    components::{Icon, Introduction},
    IconName,
};
use std::collections::{HashMap, HashSet};
use tttod_data::Player;
use uuid::Uuid;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::HtmlElement;
use ybc::{HeaderSize, TileCtx, TileSize};
use yew::prelude::*;

pub struct Lobby {
    link: ComponentLink<Self>,
    props: Props,
    input_ref: NodeRef,
    player_name: String,
    keyup_closure: Closure<dyn FnMut(web_sys::KeyboardEvent)>,
    loading: bool,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub set_name: Callback<String>,
    pub set_ready: Callback<()>,
    pub vote_kick: Callback<Uuid>,
    pub player_id: Uuid,
    pub players: HashMap<Uuid, Player>,
    pub player_kick_votes: HashMap<Uuid, HashSet<Uuid>>,
}

pub enum Msg {
    UpdateName(String),
    VoteKick(Uuid),
    EnterGame,
}

impl Component for Lobby {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let inner_link = link.clone();
        let keyup_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                inner_link.send_message(Msg::EnterGame);
                event.stop_propagation();
            }
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
        Self {
            link,
            props,
            input_ref: NodeRef::default(),
            player_name: "".to_owned(),
            keyup_closure,
            loading: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateName(name) => {
                if !self.loading {
                    self.player_name = name;
                    self.props.set_name.emit(self.player_name.clone());
                    true
                } else {
                    false
                }
            }
            Msg::EnterGame => {
                if !self.loading {
                    self.props.set_ready.emit(());
                    self.loading = true;
                    true
                } else {
                    false
                }
            }
            Msg::VoteKick(id) => {
                self.props.vote_kick.emit(id);
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if let Some(player) = props.players.get(&props.player_id) {
            self.loading = player.ready;
            self.player_name = player.name.clone();
        }
        self.props = props;
        true
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(node) = self.input_ref.get() {
                if let Some(element) = node.dyn_ref::<HtmlElement>() {
                    element.focus().ok();
                    element.set_onkeyup(self.keyup_closure.as_ref().dyn_ref());
                }
            }
        }
    }

    fn view(&self) -> Html {
        let game_callback = self.link.callback(|_| Msg::EnterGame);
        let update_name_callback = self.link.callback(Msg::UpdateName);
        html! {
            <>
                <ybc::Tile vertical=true size=TileSize::Eight ctx=TileCtx::Parent>
                    <Introduction/>
                </ybc::Tile>
                <ybc::Tile vertical=true ctx=TileCtx::Parent>
                    <ybc::Tile ctx=TileCtx::Child size=TileSize::Twelve>
                        <ybc::Section>
                            <ybc::Field classes="control has-icons-left">
                            <ybc::Input disabled=self.loading name="game" update=update_name_callback value=self.player_name.clone() placeholder="Player name" rounded=false ref=self.input_ref.clone()/>
                            <span class="icon is-small is-left">
                                    <Icon name=IconName::User/>
                                </span>
                        </ybc::Field>
                            <ybc::Field>
                                <ybc::Button loading=self.loading disabled=self.player_name.is_empty() onclick=game_callback>{"Face the Evil"}</ybc::Button>
                            </ybc::Field>
                        </ybc::Section>
                    </ybc::Tile>
                    <ybc::Tile classes="box" ctx=TileCtx::Child>
                        <ybc::Title size=HeaderSize::Is4>{"Players"}</ybc::Title>
                        <ybc::Table striped=true narrow=true fullwidth=true>
                            <thead>
                                <tr><th></th><th class="name">{"Name"}</th><th></th></tr>
                            </thead>
                            <tbody>
                            {
                                for self.props.players.iter().map(move |(player_id, player)| {
                                    let player_id = *player_id;
                                    let onclick_callback = self.link.callback(move |_| Msg::VoteKick(player_id));
                                    if player.name.is_empty() {
                                        html! {
                                            <tr><td><Icon name=IconName::Hourglass/></td><td class="name"><em>{"unknown"}</em></td><td>
                                            {
                                                if let Some(kick_votes) = self.props.player_kick_votes.get(&player_id) {
                                                    (0..kick_votes.len()).map(|_| html! {
                                                        <Icon name=IconName::SkullCrossbones/>
                                                    }).collect()
                                                } else {
                                                    html! { <></> }
                                                }
                                            }
                                            {
                                                if player_id != self.props.player_id {
                                                    html! {
                                                        <ybc::Button classes="is-danger is-rounded is-light" onclick=onclick_callback><Icon name=IconName::UserSlash/></ybc::Button>
                                                    }
                                                } else {
                                                    html! { <></> }
                                                }
                                            }
                                            </td></tr>
                                        }
                                    } else if player_id != self.props.player_id {
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
                                            }</td><td class="name">{&player.name}</td><td><ybc::Button classes="is-danger is-rounded is-light" onclick=onclick_callback><Icon name=IconName::UserSlash/></ybc::Button></td></tr>
                                        }
                                    } else {
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
                                            }</td><td class="name">{&player.name}</td><td></td></tr>
                                        }
                                    }
                                })
                            }
                            </tbody>
                        </ybc::Table>
                    </ybc::Tile>
                </ybc::Tile>
            </>
        }
    }
}
