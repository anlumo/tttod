use super::{
    setup::{Player, SelectGame},
    Icon,
};
use crate::IconName;
use ybc::{
    Navbar, NavbarDivider, NavbarDropdown,
    NavbarFixed::Top,
    NavbarItem,
    NavbarItemTag::{Div, A},
};
use yew::prelude::*;
use yew_router::{router::Router, Switch};

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/game/{game_name}"]
    Game(String),
    #[to = "/"]
    Index,
}

pub struct Root {
    link: ComponentLink<Self>,
    state: GameState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        let state = self.state;
        html! {
            <>
                <ybc::Navbar navbrand=self.view_navbrand() navstart=self.view_navstart() navend=self.view_navend()/>
                <ybc::Container fluid=false>
                    <Router<AppRoute, ()>
                        render=
                            Router::render(move |switch| {
                                match switch {
                                    AppRoute::Game(game_name) => {
                                        match state {
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
                                    AppRoute::Index => {
                                        html! {
                                            <SelectGame/>
                                        }
                                    }
                                }
                            })
                        />
                </ybc::Container>
            </>
        }
    }
}

impl Root {
    fn view_navbrand(&self) -> Html {
        html! {
            <>
                <ybc::NavbarItem tag=A classes="logo" href="/">
                    <Icon name=IconName::Gopuram/>
                    {"To the Temple of Doom!"}
                </ybc::NavbarItem>
            </>
        }
    }

    fn view_navstart(&self) -> Html {
        html! {
            <>
                <ybc::NavbarItem tag=A href="/">
                    { "Home" }
                </ybc::NavbarItem>
                <ybc::NavbarItem tag=A href="https://github.com/anlumo/tttod/blob/main/README.md">
                    { "About" }
                </ybc::NavbarItem>
                { self.view_navdrop() }
            </>
        }
    }

    fn view_navend(&self) -> Html {
        html! {
            <ybc::NavbarItem tag=A href="https://storybrewersroleplaying.com/temple-of-doom/">
                { "by Storybrewers Roleplaying" }
            </ybc::NavbarItem>
        }
    }

    fn view_navdrop(&self) -> Html {
        html! {
            <ybc::NavbarDropdown navlink=self.view_navlink() hoverable=true>
                <ybc::NavbarItem tag=A href="https://github.com/anlumo/tttod/">
                    { "Github Page" }
                </ybc::NavbarItem>
                <ybc::NavbarDivider />
                <ybc::NavbarItem tag=A href="https://github.com/anlumo/tttod/issues">
                    { "Report an issue" }
                </ybc::NavbarItem>
            </ybc::NavbarDropdown>
        }
    }

    fn view_navlink(&self) -> Html {
        html! {
            { "More" }
        }
    }
}
