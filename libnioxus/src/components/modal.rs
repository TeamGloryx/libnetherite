use dioxus::prelude::*;

#[derive(Clone, Debug, Props)]
pub struct ModalProps<'a> {
    pub open: &'a bool,
    pub header: Option<Element<'a>>,
    pub children: Element<'a>,
}

#[derive(Clone, Debug, Props)]
pub struct DefaultModalProps<'a> {
    pub open: &'a UseState<bool>,
    pub children: Element<'a>,
}

const DIALOG_STYLE: &str = include_str!("./modal.css");

pub fn DefaultModal<'a>(cx: Scope<'a, DefaultModalProps<'a>>) -> Element<'a> {
    let DefaultModalProps { open, children } = cx.props;
    cx.render(rsx! {
        Modal {
            open: (*open).get(),
            header: cx.render(rsx! {
                div { style: "display: flex; justify-content: flex-end;",
                    button { style: "font-size: 24px", onclick: move |_| (*open).set(false),
                             "x"
                    }
                }
            }),
            children
        }
    })
}

pub fn Modal<'a>(cx: Scope<'a, ModalProps<'a>>) -> Element<'a> {
    let ModalProps {
        open,
        header,
        children,
    } = cx.props;

    cx.render(rsx! {
        dialog {
            open: **open,

            header.as_ref().map(|header| rsx! {
                header
                hr {}
            })

            children
        }
        style {
            DIALOG_STYLE
        }
    })
}
