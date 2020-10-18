mod player;
pub use player::Player;

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
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub game_name: String,
}

pub enum Msg {
    SetPlayerName(String),
}

impl Component for Game {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            state: GameState::PlayerSelection,
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

    fn view(&self) -> Html {
        let set_name_callback = self.link.callback(Msg::SetPlayerName);
        html! {
            <ybc::Tile classes="top-level" vertical=false>
            {
                match self.state {
                    GameState::PlayerSelection => {
                        html! {
                            <Player set_name=set_name_callback/>
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
