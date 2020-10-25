use yew::prelude::*;

pub struct ModalDialog {
    props: Props,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    /// The ID of this modal, used for triggering close events from other parts of the app.
    pub id: String,
    /// The title of this modal.
    pub title: String,
    /// The content to be placed in the `modal-card-body` not including the modal-card-header /
    /// modal-card-title, which is handled by the `modal_title` prop.
    #[prop_or_default]
    pub body: Html,
    /// The content to be placed in the `modal-card-footer`.
    #[prop_or_default]
    pub footer: Html,
    /// The contents of the modal trigger, typically a button or the like.
    #[prop_or_default]
    pub trigger: Html,
    #[prop_or_default]
    pub classes: Option<String>,
    pub is_active: bool,
    #[prop_or_default]
    pub close_callback: Option<Callback<()>>,
}

impl Component for ModalDialog {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("modal");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if self.props.is_active {
            classes.push("is-active");
        }
        html! {
            <div id=self.props.id.clone() class=classes>
                {
                    if let Some(close_callback) = &self.props.close_callback {
                        html! {
                            <div class="modal-background" onclick=close_callback.reform(|_| ())></div>
                        }
                    } else {
                        html! {
                            <div class="modal-background"></div>
                        }
                    }
                }
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{self.props.title.clone()}</p>
                        {
                            if let Some(close_callback) = &self.props.close_callback {
                                html! {
                                    <button class="delete" aria-label="close" onclick=close_callback.reform(|_| ())></button>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </header>
                    <section class="modal-card-body">
                        {self.props.body.clone()}
                    </section>
                    <footer class="modal-card-foot">
                        {self.props.footer.clone()}
                    </footer>
                </div>
                {
                    if let Some(close_callback) = &self.props.close_callback {
                        html! {
                            <button class="modal-close is-large" aria-label="close" onclick=close_callback.reform(|_| ())></button>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    }
}
