use crate::components::Introduction;
use ybc::{
    HeaderSize,
    NavbarFixed::Top,
    TileCtx::{Ancestor, Child, Parent},
    TileSize, Title,
};
use yew::prelude::*;

pub struct Player {
    link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for Player {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
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
            <ybc::Tile classes="top-level" vertical=false>
                <Introduction/>
                <ybc::Tile vertical=true  size=TileSize::Four>
                    <ybc::Section>
                        {"Player name:"}
                        <input type="text" id="player"/>
                        <button type="submit">{"Enter the Temple"}</button>
                    </ybc::Section>
                </ybc::Tile>
            </ybc::Tile>
        }
    }
}
