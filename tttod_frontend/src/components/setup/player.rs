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
            <main>
                <h1>{"To the Temple of Doom!"}</h1>
                <h2>{"To Defeat the Ancient Evil!"}</h2>
                <div>{"by Hayley Gordon and Vee Hendro"}</div>
                <section>
                    <h3>{"You are an Expert Archeologist…"}</h3>
                    <p>
                        {"In fact, you're the best in your field. Until now you've spent your days \
                        buried in musty tomes, toiling on dig sites, and putting artifacts in \
                        museums to save the past. Now, you must save the futures."}
                    </p>
                </section>
                <section>
                    <h3>{"An Ancient Evil Stirs…"}</h3>
                    <p>
                        {"It wakes deep within the bowels of an untouched temple. An evil that will \
                        end the world as we know it. Only you and your fellow archeologists can \
                        examine the clues, unravel the mysteries, and uncover the method to subdue \
                        this terrible thread."}
                    </p>
                </section>
                <section>
                    <h3>{"Enter the Temple of Doom…"}</h3>
                    <p>
                        {"Find its secrets, and defeat the ancient evil before it destroys the world!"}
                    </p>
                </section>
                <div>
                    {"Player name:"}
                    <input type="text" id="player"/>
                    <button type="submit">{"Enter the Temple"}</button>
                </div>
            </main>
        }
    }
}
