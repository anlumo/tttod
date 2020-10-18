use super::setup::Player;
use yew::prelude::*;

pub struct Root {
    link: ComponentLink<Self>,
    state: GameState,
}

pub enum GameState {
    PlayerSelection,
    QuestionsAndAnswers,
    CharacterCreation,
    Game,
}

pub enum Msg {}

impl Component for Root {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            state: GameState::PlayerSelection,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // match msg {
        // }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
            {
                match self.state {
                    GameState::PlayerSelection => {
                        html! {
                            <Player/>
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
            </div>
        }
    }
}
