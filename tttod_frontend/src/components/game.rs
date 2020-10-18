mod lobby;
pub use lobby::Lobby;

use uuid::Uuid;
use yew::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    PlayerSelection,
    QuestionsAndAnswers,
    CharacterCreation,
    Game,
}

pub struct Game {
    link: ComponentLink<Self>,
    props: Props,
    state: GameState,
    player_id: Uuid,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub game_name: String,
}

pub enum Msg {
    SetPlayerName(String),
}

fn local_storage() -> web_sys::Storage {
    web_sys::window().unwrap().local_storage().unwrap().unwrap()
}

impl Component for Game {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = local_storage();
        let player_id = if let Some(player_id) = storage
            .get_item("player_id")
            .unwrap()
            .and_then(|player_id| Uuid::parse_str(&player_id).ok())
        {
            player_id
        } else {
            let player_id = Uuid::new_v4();
            storage
                .set_item("player_id", &format!("{}", player_id))
                .unwrap();
            player_id
        };
        Self {
            link,
            props,
            state: GameState::PlayerSelection,
            player_id,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetPlayerName(name) => {
                // TODO
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            // TODO
        }
    }

    fn view(&self) -> Html {
        let set_name_callback = self.link.callback(Msg::SetPlayerName);
        html! {
            <ybc::Tile classes="top-level" vertical=false>
            {
                match self.state {
                    GameState::PlayerSelection => {
                        html! {
                            <Lobby set_name=set_name_callback/>
                        }
                    }
                    GameState::QuestionsAndAnswers => {
                        html! {
                            <div/>
                        }
                    }
                    GameState::CharacterCreation => {
                        html! {
                            <div/>
                        }
                    }
                    GameState::Game => {
                        html! {
                            <div/>
                        }
                    }
                }
            }
            </ybc::Tile>
        }
    }
}
