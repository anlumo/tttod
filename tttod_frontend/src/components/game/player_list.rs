use crate::{components::Icon, IconName};
use std::collections::HashMap;
use tttod_data::Player;
use uuid::Uuid;
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
            <div class="player-list">
            {
                for self.props.players.iter().map(|(player_id, player)| {
                    html! {
                        <span title={&player.name}>
                            <Icon name={if player.ready { IconName::UserGraduate } else { IconName::UserClock }} classes={
                                if player.ready {
                                    "user-ready"
                                } else {
                                    "user-not-ready"
                                }
                            }/>
                        </span>
                    }
                })
            }
            </div>
        }
    }
}
