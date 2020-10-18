use super::{game::Game, setup::SelectGame, Icon};
use crate::IconName;
use ybc::NavbarItemTag::A;
use yew::prelude::*;
use yew_router::{router::Router, Switch};

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/game/{game_name}"]
    Game(String),
    #[to = "/"]
    Index,
}

pub struct Root;

pub enum Msg {}

impl Component for Root {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <ybc::Navbar navbrand=self.view_navbrand() navstart=self.view_navstart() navend=self.view_navend()/>
                <ybc::Container fluid=false>
                    <Router<AppRoute, ()>
                        render=
                            Router::render(move |switch| {
                                match switch {
                                    AppRoute::Game(game_name) => {
                                        html! {
                                            <Game game_name=game_name/>
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
                { "RPG by Storybrewers Roleplaying" }
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
