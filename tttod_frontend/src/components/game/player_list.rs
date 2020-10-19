use crate::{components::Icon, IconName};
use std::collections::HashMap;
use tttod_data::Player;
use uuid::Uuid;
use ybc::{HeaderSize, TileCtx};
use yew::prelude::*;

pub struct PlayerList {
    props: Props,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub player_id: Uuid,
    pub players: HashMap<Uuid, Player>,
}

impl Component for PlayerList {
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
        html! {
            <ybc::Tile classes="box" ctx=TileCtx::Child>
                <ybc::Title size=HeaderSize::Is4>{"Players"}</ybc::Title>
                <ybc::Table striped=true narrow=true fullwidth=true>
                    <thead>
                        <tr><th></th><th class="name">{"Name"}</th></tr>
                    </thead>
                    <tbody>
                    {
                        for self.props.players.iter().map(move |(player_id, player)| {
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
                                }</td><td class="name">{&player.name}</td></tr>
                            }
                        })
                    }
                    </tbody>
                </ybc::Table>
            </ybc::Tile>
        }
    }
}
