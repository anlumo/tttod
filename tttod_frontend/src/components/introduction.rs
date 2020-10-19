use ybc::{HeaderSize, TileCtx};
use yew::prelude::*;

pub struct Introduction;

impl Component for Introduction {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
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
            <ybc::Tile vertical=true ctx=TileCtx::Child>
                <ybc::Title size=HeaderSize::Is1>{"To the Temple of Doom!"}</ybc::Title>
                <ybc::Title size=HeaderSize::Is3>{"To Defeat the Ancient Evil!"}</ybc::Title>
                <ybc::Section>
                    <ybc::Title size=HeaderSize::Is4>{"You are an Expert Archeologist…"}</ybc::Title>
                    <p>
                        {"In fact, you're the best in your field. Until now you've spent your days \
                        buried in musty tomes, toiling on dig sites, and putting artifacts in \
                        museums to save the past. Now, you must save the futures."}
                    </p>
                </ybc::Section>
                <ybc::Section>
                    <ybc::Title size=HeaderSize::Is4>{"An Ancient Evil Stirs…"}</ybc::Title>
                    <p>
                        {"It wakes deep within the bowels of an untouched temple. An evil that will \
                        end the world as we know it. Only you and your fellow archeologists can \
                        examine the clues, unravel the mysteries, and uncover the method to subdue \
                        this terrible thread."}
                    </p>
                </ybc::Section>
                <ybc::Section>
                    <ybc::Title size=HeaderSize::Is4>{"Enter the Temple of Doom…"}</ybc::Title>
                    <p>
                        {"Find its secrets, and defeat the ancient evil before it destroys the world!"}
                    </p>
                </ybc::Section>
            </ybc::Tile>
        }
    }
}
