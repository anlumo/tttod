use super::{CharacterViewer, PlayerList};
use std::collections::HashMap;
use tttod_data::Player;
use uuid::Uuid;
use ybc::{HeaderSize, TileCtx, TileSize};
use yew::prelude::*;

pub struct IntroduceCharacters {
    link: ComponentLink<Self>,
    props: Props,
    loading: bool,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub player_id: Uuid,
    pub players: HashMap<Uuid, Player>,
    pub set_ready: Callback<()>,
}

pub enum Msg {
    Ready,
}

impl Component for IntroduceCharacters {
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
                self.loading = true;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let ready_callback = self.link.callback(|_| Msg::Ready);
        let player = self.props.players.get(&self.props.player_id);
        html! {
            <ybc::Tile vertical=false ctx=TileCtx::Parent>
                <ybc::Tile vertical=false ctx=TileCtx::Child size=TileSize::Eight>
                    <ybc::Title size=HeaderSize::Is1>{"Introduce Your Archeologists"}</ybc::Title>
                    <p class="block">{"Answer these questions, and add any more details you wish. You may decide \
                    to explore these questions through a short scene between the archeologists outside the temple."}</p>
                    <ul class="block">
                        <li>{"Who are you?"}</li>
                        <li>{"Why did you decide to come?"}</li>
                        <li>{"What do you think of your team? Have you met before?"}</li>
                        <li>{"How do you feel about the task ahead?"}</li>
                    </ul>
                    {
                        if let Some(player) = player {
                            html! {
                                <CharacterViewer player=player/>
                            }
                        } else {
                            html! { <> </> }
                        }
                    }
                </ybc::Tile>
                <ybc::Tile vertical=true ctx=TileCtx::Parent>
                    <ybc::Tile ctx=TileCtx::Child size=TileSize::Twelve>
                        <ybc::Button loading=self.loading onclick=ready_callback>{"Enter the Temple"}</ybc::Button>
                    </ybc::Tile>
                    <PlayerList player_id=self.props.player_id players=&self.props.players/>
                </ybc::Tile>
            </ybc::Tile>
        }
    }
}
