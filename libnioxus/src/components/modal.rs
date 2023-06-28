use dioxus::prelude::*;

#[derive(Clone, Debug, Props)]
pub struct ModalProps<'a> {
    pub open: &'a bool,
    pub children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Modal<'a>(cx: Scope<'a, ModalProps<'a>>) -> Element<'a> {
    let ModalProps { open, children } = cx.props;

    cx.render(rsx! {
        dialog {
            open: **open,

            children
        }
    })
}
