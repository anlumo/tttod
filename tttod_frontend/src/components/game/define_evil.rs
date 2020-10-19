use crate::{components::Icon, IconName};
use std::collections::HashMap;
use tttod_data::Player;
use uuid::Uuid;
use ybc::{HeaderSize, Size, TileCtx, TileSize};
use yew::prelude::*;

pub struct DefineEvil {
    link: ComponentLink<Self>,
    props: Props,
    loading: bool,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub player_id: Uuid,
    pub players: HashMap<Uuid, Player>,
    pub questions: Vec<(String, String)>,
    pub set_answer: Callback<(usize, String)>,
    pub set_ready: Callback<()>,
}

pub enum Msg {
    SetAnswer(usize, String),
    Ready,
}

impl Component for DefineEvil {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            loading: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetAnswer(idx, text) => {
                self.props.set_answer.emit((idx, text));
            }
            Msg::Ready => {
                self.props.set_ready.emit(());
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let ready_callback = self.link.callback(|_| Msg::Ready);
        html! {
            <ybc::Tile vertical=false ctx=TileCtx::Parent>
                <ybc::Tile vertical=false ctx=TileCtx::Child size=TileSize::Eight>
                    <ybc::Title size=HeaderSize::Is1>{"Define the Evil"}</ybc::Title>
                    <p class="block">{"Create a powerful malignant force for the heroes to contend with. Secretly answer the following questions. \
                    Answers should be complete self-contained sentences, written in first person from the perspective of the ancient \
                    evil. Click the button on the right when you're done."}</p>
                    {
                        for self.props.questions.iter().enumerate().map(|(idx, (question, answer))| {
                            let update_callback = self.link.callback(move |text| Msg::SetAnswer(idx, text));
                            html! {
                                <ybc::Box>
                                    <ybc::Title size=HeaderSize::Is4>{question}</ybc::Title>
                                    <ybc::TextArea name={format!("q{}", idx+1)} rows=5 disabled=self.loading value=answer.clone() placeholder="Answer" update=update_callback size=Size::Medium/>
                                </ybc::Box>
                            }
                        })
                    }
                </ybc::Tile>
                <ybc::Tile vertical=true ctx=TileCtx::Parent>
                    <ybc::Tile ctx=TileCtx::Child size=TileSize::Twelve>
                        <ybc::Button loading=self.loading disabled=self.props.questions.iter().any(|(q, a)| a.is_empty()) onclick=ready_callback>{"Look Into the Mirror"}</ybc::Button>
                    </ybc::Tile>
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
                </ybc::Tile>
            </ybc::Tile>
        }
    }
}
